mod handlers;

use crate::{core::{self,
                   fs::{launcher_root_path,
                        FS_ROOT_PATH},
                   os::{process,
                        signals},
                   package::{PackageIdent,
                             PackageInstall}},
            protocol::{self,
                       ERR_NO_RETRY_EXCODE,
                       OK_NO_RETRY_EXCODE},
            server::handlers::Handler,
            service::Service,
            SUP_CMD,
            SUP_PACKAGE_IDENT};
use anyhow::{anyhow,
             Context,
             Result};
use biome_common::{liveliness_checker::{self,
                                          ThreadUnregistered},
                     outputln};
#[cfg(unix)]
use biome_core::os::process::{Pid,
                                Signal};
use ipc_channel::ipc::{IpcOneShotServer,
                       IpcReceiver,
                       IpcSender};
use log::{debug,
          error,
          warn};
use semver::{Version,
             VersionReq};
#[cfg(unix)]
use std::{cmp::Ordering,
          os::unix::process::ExitStatusExt};
use std::{collections::HashMap,
          fs,
          io::Write,
          path::PathBuf,
          process::{Child,
                    Command,
                    ExitStatus,
                    Stdio},
          str::FromStr,
          sync::{Arc,
                 Condvar,
                 Mutex},
          thread,
          time::Duration};

const IPC_CONNECT_TIMEOUT_SECS: &str = "HAB_LAUNCH_SUP_CONNECT_TIMEOUT_SECS";
const DEFAULT_IPC_CONNECT_TIMEOUT_SECS: u64 = 5;
const SUP_CMD_ENVVAR: &str = "HAB_SUP_BINARY";
static LOGKEY: &str = "SV";

const SUP_VERSION_CHECK_DISABLE: &str = "HAB_LAUNCH_NO_SUP_VERSION_CHECK";
// Version 0.56 is somewhat arbitrary. This functionality is for when we make
// changes to the launcher that depend on supervisor behavior that hasn't
// always existed such as https://github.com/habitat-sh/habitat/issues/5380
const SUP_VERSION_REQ: &str = ">= 0.56";

type Receiver = IpcReceiver<Vec<u8>>;
type Sender = IpcSender<Vec<u8>>;

enum TickState {
    Continue,
    Exit(i32),
}

pub struct Server {
    pid_file_path: PathBuf,
    services:      ServiceTable,
    tx:            Sender,
    rx:            Receiver,
    supervisor:    Child,
    args:          Vec<String>,
}

impl Drop for Server {
    fn drop(&mut self) { fs::remove_file(&self.pid_file_path).ok(); }
}

impl Server {
    pub fn new(args: Vec<String>) -> Result<Self> {
        let launcher_root = launcher_root_path(Some(&*core::fs::FS_ROOT_PATH));
        fs::create_dir_all(&launcher_root).with_context(|| {
                                              format!("Failed to create the launcher runtime \
                                                       folder '{}'",
                                                      launcher_root.display())
                                          })?;
        let pid_file_path = launcher_root.join("PID");
        let mut pid_file = fs::File::create(&pid_file_path).with_context(|| {
                                                               format!("Failed to create the \
                                                                        launcher PID file '{}'",
                                                                       pid_file_path.display())
                                                           })?;
        write!(&mut pid_file, "{}", process::current_pid()).with_context(|| {
                                                               format!("Failed to write launcher \
                                                                        pid to PID file '{}'",
                                                                       pid_file_path.display())
                                                           })?;

        let ((rx, tx), supervisor) = Self::init(&args).context("Failed to initialize launcher")?;
        Ok(Server { pid_file_path,
                    services: ServiceTable::default(),
                    tx,
                    rx,
                    supervisor,
                    args })
    }

    /// Spawn a Supervisor and setup a bi-directional IPC connection to it.
    ///
    /// Passing a value of true to the `clean` argument will force the Supervisor to clean the
    /// Launcher's process LOCK before starting. This is useful when restarting a Supervisor
    /// that terminated gracefully.
    fn init(args: &[String]) -> Result<((Receiver, Sender), Child)> {
        let (server, pipe) =
            IpcOneShotServer::new().context("Failed to create incoming IPC channel for launcher")?;
        let supervisor = spawn_supervisor(&pipe, args).context("Failed to spawn supervisor")?;
        let ipc_channel = setup_connection(server).context("Failed to setup launcher IPC \
                                                            connection with supervisor")?;
        Ok((ipc_channel, supervisor))
    }

    #[allow(unused_must_use)]
    fn reload(&mut self) -> Result<()> {
        self.supervisor.kill();
        self.supervisor.wait();
        let ((rx, tx), supervisor) = Self::init(&self.args).context("Failed to reload launcher")?;
        self.tx = tx;
        self.rx = rx;
        self.supervisor = supervisor;
        Ok(())
    }

    // Signals aren't a thing on Windows
    #[cfg(unix)]
    fn forward_signal(&self, signal: Signal) {
        if let Err(err) = core::os::process::signal(self.supervisor.id() as Pid, signal) {
            error!("Unable to signal Supervisor, {}, {}",
                   self.supervisor.id(),
                   err);
        }
    }

    fn handle_message(&mut self) -> Result<TickState> {
        match self.rx.try_recv() {
            Ok(bytes) => {
                dispatch(&self.tx, &bytes, &mut self.services);
                Ok(TickState::Continue)
            }
            Err(_) => {
                match self.supervisor.try_wait() {
                    Ok(None) => Ok(TickState::Continue),
                    Ok(Some(status)) => {
                        // Supervisor exited
                        self.handle_supervisor_exit(status)
                    }
                    Err(err) => {
                        warn!("Failed to wait for supervisor process: {}", err);
                        Err(anyhow!("Failed to wait for supervisor process to exit"))
                    }
                }
            }
        }
    }

    /// Given that a Supervisor process has exited with a specific
    /// exit code, figure out whether we need to restart it or not.
    fn handle_supervisor_exit(&mut self, status: ExitStatus) -> Result<TickState> {
        let code = status.code();
        match code {
            Some(ERR_NO_RETRY_EXCODE) => {
                self.services.kill_all();
                Ok(TickState::Exit(ERR_NO_RETRY_EXCODE))
            }
            Some(OK_NO_RETRY_EXCODE) => {
                self.services.kill_all();
                Ok(TickState::Exit(0))
            }
            Some(exit_code) => {
                Err(anyhow!("Supervisor process exited with an unexpected \
                             exit code: {}",
                            exit_code))
            }
            None => {
                #[cfg(unix)]
                {
                    match status.signal() {
                        Some(signal) => {
                            outputln!("Supervisor process killed by signal {}; shutting \
                                       everything down now",
                                      signal);
                            self.services.kill_all();
                            Ok(TickState::Exit(0))
                        }
                        None => {
                            Err(anyhow!("Supervisor process was terminated in some unknown manner"))
                        }
                    }
                }
                // This branch is essentially unreachable as the underlying ExitStatus
                // implementation for windows will always return some error code.
                #[cfg(not(unix))]
                {
                    self.services.kill_all();
                    Ok(TickState::Exit(0))
                }
            }
        }
    }

    fn reap_services(&mut self) { self.services.reap_services() }

    fn shutdown(&mut self) {
        debug!("Shutting down launcher");
        match send(&self.tx, &protocol::Shutdown::default()) {
            Ok(_) => {}
            Err(err) => {
                debug!("Failed to shutdown supervisor process with pid {}: {:?}",
                       self.supervisor.id(),
                       err);
                warn!("Forcefully stopping supervisor: {}", self.supervisor.id());
                if let Err(err) = self.supervisor.kill() {
                    warn!("Unable to kill supervisor, {}, {}",
                          self.supervisor.id(),
                          err);
                }
            }
        }

        // With the Supervisor shutting down services, we need to
        // ensure that the Launcher is able to reap those main service
        // processes as the Supervisor shuts them down... otherwise,
        // the Supervisor can end up waiting a long time on zombie
        // processes.
        //
        // But see https://github.com/habitat-sh/habitat/issues/6131
        // for a possible future where this isn't needed, and reaping
        // could theoretically just take place at the very end of this
        // shutdown process, rather than repeatedly.
        //
        // TODO (CM): need some kind of timeout here... if the
        // Supervisor didn't get the signal, then we're just going to
        // hang here forever. We should have a (customizable) timeout
        // here, after which we kill *everything*.
        while let Ok(None) = self.supervisor.try_wait() {
            self.services.reap_services();
            thread::sleep(Duration::from_millis(5));
        }

        match self.supervisor.try_wait() {
            Ok(Some(status)) => debug!("Supervisor exited with: {}", status),
            Err(e) => error!("Error waiting on supervisor: {:?}", e),
            _ => unreachable!(),
        }

        // TODO (CM): Eventually this can go away... but we need to
        // keep it around while we still support older Supervisors
        // that don't shutdown services themselves.
        self.services.kill_all();
        outputln!("Hasta la vista, services.");
    }

    fn tick(&mut self) -> Result<TickState> {
        // TODO (CM): Yes, we have `reap_services` as well as
        // `reap_zombie_orphans`... perhaps they need different
        // names. However, this is a distinction that might be nice to
        // collapse in the future.
        //
        // `reap_services` is a cross-platform method to reap (and keep
        // track of) processes that are Biome
        // services. `reap_zombie_orphans` is basically a Unix-only
        // method to take care of any orphan processes that get
        // re-parented to the Launcher, when it is running as PID 1,
        // when their parents end before they do.
        //
        // There is some natural overlap between the two on Unix
        // platforms that would be nice to collapse, but it needs to
        // be done in a way that the basic functionality of process
        // tracking still works on Windows.
        self.reap_services();

        if signals::pending_shutdown() {
            self.shutdown();
            return Ok(TickState::Exit(0));
        }

        #[cfg(unix)]
        {
            if signals::pending_sigchld() {
                // We only return Some if we ended up reaping our
                // Supervisor; otherwise, we don't need to do anything
                // special. If the supervisor exits but reap_zombie_orphans()
                // doesn't catch the signal (such as on Windows), we will still
                // handle that properly in handle_message().
                if let Some(result) = self.reap_zombie_orphans() {
                    return result;
                }
            }

            if signals::pending_sighup() {
                self.forward_signal(Signal::HUP);
            }
        }
        self.handle_message()
    }

    /// When the supervisor runs as the init process (e.g. in a
    /// container), it will become the parent of any processes whose
    /// parents terminate before they do (as is standard on Linux). We
    /// need to call `waitpid` on these children to prevent a zombie
    /// horde from ultimately bringing down the system.
    ///
    /// Note that we are not (yet?) doing anything with
    /// `prctl(PR_SET_CHILD_SUBREAPER, ...)` to make the Launcher a
    /// subreaper; this behavior currently handles the case when the
    /// Launcher is running as PID 1.
    ///
    /// (See http://man7.org/linux/man-pages/man2/prctl.2.html for
    /// further information.)
    #[cfg(unix)]
    fn reap_zombie_orphans(&mut self) -> Option<Result<TickState>> {
        // Record the disposition of the Supervisor if it is a child
        // process being reaped; our ultimate response is dependent on
        // this.
        let mut reaped_sup_status: Option<ExitStatus> = None;
        let mut waitpid_status = 0;

        // We reap as many child processes as need reaping.
        loop {
            // We're not calling waitpid with WUNTRACED or WCONTINUED,
            // so we shouldn't be getting SIGCHLD from STOP or CONT
            // signals sent to a Supervisor; only when the Supervisor
            // process ends somehow.
            let res = unsafe { libc::waitpid(-1, &mut waitpid_status, libc::WNOHANG) };
            match res.cmp(&0) {
                Ordering::Greater => {
                    // Some child process ended; let's see if it was the Supervisor
                    if res == self.supervisor.id() as libc::pid_t {
                        debug!("Reaped supervisor process, PID {}", res);
                        // Note: from_raw is a Unix-only call
                        reaped_sup_status = Some(ExitStatus::from_raw(waitpid_status));
                    } else {
                        debug!("Reaped a non-supervisor child process, PID {}", res);
                    }
                }
                Ordering::Less => {
                    warn!("Error waiting for child process: {}", res);
                    break;
                }
                Ordering::Equal => {
                    // There are no more children waiting
                    break;
                }
            }
        }

        // If we reaped our supervisor, then we return a TickState so
        // we can figure out whether or restart or not.
        //
        // If we just reaped non-supervisor processes, though, we
        // return `None` to indicate there's nothing special that
        // needs to happen.
        reaped_sup_status.map(|status| self.handle_supervisor_exit(status))
    }
}

#[derive(Debug, Default)]
pub struct ServiceTable(HashMap<u32, Service>);

impl ServiceTable {
    pub fn get(&self, pid: u32) -> Option<&Service> { self.0.get(&pid) }

    pub fn get_mut(&mut self, pid: u32) -> Option<&mut Service> { self.0.get_mut(&pid) }

    pub fn insert(&mut self, service: Service) { self.0.insert(service.id(), service); }

    pub fn remove(&mut self, pid: u32) -> Option<Service> { self.0.remove(&pid) }

    // Obviously this is not the most elegant implementation. However,
    // in practice we don't have a whole lot of processes per
    // Supervisor. A better-than-O(n) solution would also require more
    // extensive refactoring of this data type, which is not something
    // I particularly want to dive into *right now* (eventually,
    // though).
    //
    // Note that this also implicitly relies on the fact that a single
    // supervisor can't be running more than one service with a given
    // name. That should always be the case, but *this* code doesn't
    // enforce that.
    //
    // TODO (CM): Enforce that service_name is actually a full
    // ServiceGroup name (and elsewhere)
    /// Given the name of a service group (e.g. "redis.default"),
    /// return the PID of the process we're currently running for that
    /// service group, if it exists.
    ///
    /// This allows a restarting Supervisor to query the Launcher to
    /// figure out if there are currently-running services to which it
    /// needs to re-attach itself.
    pub fn pid_of(&self, service_name: &str) -> Option<u32> {
        self.0.iter().find_map(|(pid, service)| {
                         if service_name == service.args().id {
                             Some(*pid)
                         } else {
                             None
                         }
                     })
    }

    fn kill_all(&mut self) {
        for service in self.0.values_mut() {
            outputln!(preamble service.name(), "Stopping...");
            let shutdown_method = service.kill();
            outputln!(preamble service.name(), "Shutdown OK: {}", shutdown_method);
        }
    }

    fn reap_services(&mut self) {
        let mut dead: Vec<u32> = vec![];
        for service in self.0.values_mut() {
            match service.try_wait() {
                Ok(None) => (),
                Ok(Some(code)) => {
                    outputln!("Child for service '{}' with PID {} exited with code {}",
                              service.name(),
                              service.id(),
                              code);
                    dead.push(service.id());
                }
                Err(err) => {
                    warn!("Error waiting for child, {}, {}", service.id(), err);
                    dead.push(service.id());
                }
            }
        }
        for pid in dead {
            self.0.remove(&pid);
        }
    }
}

////////////////////////
// Public Func
//

pub fn run(args: Vec<String>) -> Result<i32> {
    let mut server = Server::new(args)?;
    liveliness_checker::spawn_thread_alive_checker();
    let loop_value: ThreadUnregistered<_, _> = loop {
        let checked_thread = liveliness_checker::mark_thread_alive();

        match server.tick() {
            Ok(TickState::Continue) => thread::sleep(Duration::from_millis(100)),
            Ok(TickState::Exit(code)) => {
                break checked_thread.unregister(Ok(code));
            }
            Err(err) => {
                error!("Launcher will attempt to reload the supervisor: {:?}", err);
                match server.reload() {
                    Ok(_) => {}
                    Err(err) => {
                        error!("Launcher failed to reload supervisor: {:?}", err);
                        thread::sleep(Duration::from_millis(1_000));
                    }
                }
            }
        }
    };
    loop_value.into_result()
}

pub fn send<T>(tx: &Sender, msg: &T) -> Result<()>
    where T: protocol::LauncherMessage
{
    let msg = protocol::NetTxn::build(msg).map_err(|err| {
                                              anyhow!("Failed to serialize launcher protocol \
                                                       message: {0}",
                                                      err)
                                          })?;
    let bytes = msg.to_bytes().map_err(|err| {
                                   anyhow!("Failed to serialize launcher protocol message \
                                            payload: {0}",
                                           err)
                               })?;
    tx.send(bytes)
      .context("Failed to send IPC message to supervisor")?;
    Ok(())
}

////////////////////////
// Private Func
//

fn dispatch(tx: &Sender, bytes: &[u8], services: &mut ServiceTable) {
    let msg = match protocol::NetTxn::from_bytes(bytes) {
        Ok(msg) => msg,
        Err(err) => {
            error!("Unable to decode NetTxn from Supervisor, {}", err);
            return;
        }
    };
    let func = match msg.message_id() {
        "Restart" => handlers::RestartHandler::run,
        "Spawn" => handlers::SpawnHandler::run,
        "Terminate" => handlers::TerminateHandler::run,
        "PidOf" => handlers::PidHandler::run,
        "Version" => handlers::VersionHandler::run,
        unknown => {
            // This sucks a bit because it replicates some code from the
            // Handler trait, but manipulating an unknown message
            // doesn't really fit that pattern :(
            let msg = format!("Received unknown message from Supervisor, {}", unknown);
            warn!("{}", msg);
            let reply = protocol::NetErr { code: protocol::ErrCode::UnknownMessage,
                                           msg };
            if let Err(err) = send(tx, &reply) {
                error!("{}: replying, {}", unknown, err);
            }
            return;
        }
    };
    func(tx, msg, services);
}

#[allow(clippy::mutex_atomic)] // A Mutex is required for Condvar::wait_timeout
fn setup_connection(server: IpcOneShotServer<Vec<u8>>) -> Result<(Receiver, Sender)> {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    // Set up the connection in a separate thread because ipc-channel doesn't support timeouts
    let handle = thread::spawn(move || {
        {
            let (ref lock, _) = *pair2;
            let mut started = lock.lock().unwrap();
            *started = true;
            debug!("connect thread started");
        }
        let (rx, raw) = server.accept()
                              .context("Failed to accept IPC connection from supervisor")?;
        let txn = protocol::NetTxn::from_bytes(&raw).map_err(|err| {
                                                        anyhow!("Failed to deserialize launcher \
                                                                 protocol 'Register' message from \
                                                                 supervisor: {}",
                                                                err)
                                                    })?;
        let msg = txn.decode::<protocol::Register>().map_err(|err| {
                                                         anyhow!("Failed to deserialize launcher \
                                                                  protocol 'Register' message \
                                                                  payload from supervisor: {}",
                                                                 err)
                                                     })?;
        let tx = IpcSender::connect(msg.pipe).context("Failed to establish IPC connection to \
                                                       the supervisor")?;
        send(&tx, &protocol::NetOk::default())?;
        {
            let (_, ref cvar) = *pair2;
            debug!("Connect thread finished; notifying waiting thread");
            cvar.notify_one();
        }
        Ok((rx, tx))
    });

    let (ref lock, ref cvar) = *pair;
    let timeout_secs =
        core::env::var(IPC_CONNECT_TIMEOUT_SECS).unwrap_or_default()
                                                .parse()
                                                .unwrap_or(DEFAULT_IPC_CONNECT_TIMEOUT_SECS);

    debug!("Waiting on connect thread for {} secs", timeout_secs);
    let (started, wait_result) = cvar.wait_timeout(lock.lock().expect("IPC connection startup \
                                                                       lock poisoned"),
                                                   Duration::from_secs(timeout_secs))
                                     .expect("IPC connection startup lock poisoned");

    if *started && !wait_result.timed_out() {
        handle.join().unwrap()
    } else {
        debug!("Timeout exceeded waiting for IPC connection (started: {})",
               *started);
        Err(anyhow!("Timeout exceeded waiting for IPC connection from supervisor"))
    }
}

/// Return whether the given version string matches SUP_VERSION_REQ parsed as
/// a semver::VersionReq.
///
/// Example inputs (that is `bio-sup --version` outputs):
/// bio-sup 0.59.0/20180712161546
/// bio-sup 0.62.0-dev
fn is_supported_supervisor_version(version_output: &str) -> bool {
    if let Some(version_str) = version_output
        .split_whitespace()                 // ["bio-sup", <version-number>]
        .last()                             // drop "bio-sup", keep <version-number>
        .unwrap()                           // split() always returns an 1+ element iterator
        .split(['/', '-'])                  // strip "-dev" or "/build"
        .next()
    {
        debug!("Checking Supervisor version '{}' against requirement '{}'",
               version_str, SUP_VERSION_REQ);
        match Version::parse(version_str) {
            Ok(version) => {
                VersionReq::parse(SUP_VERSION_REQ).expect("invalid semantic version requirement")
                                                  .matches(&version)
            }
            Err(e) => {
                error!("{}: {}", e, version_str);
                debug!("Original version command output: {}", version_output);
                false
            }
        }
    } else {
        error!("Expected 'bio-sup <semantic-version>', found '{}'",
               version_output);
        false
    }
}

/// Start a Supervisor as a child process.
///
/// Passing a value of true to the `clean` argument will force the Supervisor to clean the
/// Launcher's process LOCK before starting. This is useful when restarting a Supervisor
/// that terminated gracefully.
fn spawn_supervisor(pipe: &str, args: &[String]) -> Result<Child> {
    let binary = supervisor_cmd().context("Failed to find supervisor binary")?;

    if core::env::var(SUP_VERSION_CHECK_DISABLE).is_ok() {
        warn!("Launching Supervisor {:?} without version checking", binary);
    } else {
        debug!("Checking Supervisor {:?} version", binary);
        let version_check = Command::new(&binary).arg("--version")
                                                 .env("RUST_LOG", "error")
                                                 .output()
                                                 .context("Failed to check supervisor version")?;
        let sup_version = String::from_utf8_lossy(&version_check.stdout);
        if !is_supported_supervisor_version(sup_version.trim()) {
            error!("This Launcher requires Biome version {}", SUP_VERSION_REQ);
            error!("This check can be disabled by setting the {} environment variable to a \
                    non-empty string when starting the supervisor",
                   SUP_VERSION_CHECK_DISABLE);
            error!("Disabling this check may result in undefined behavior; please update to a \
                    newer Biome version");
            error!("For more information see https://github.com/habitat-sh/habitat/pull/5484");
            return Err(anyhow!("This launcher does not support supervisor \
                                version {}",
                               sup_version.trim()));
        }
    }

    let mut command = Command::new(&binary);

    debug!("Starting Supervisor {:?} with args {:?}, {}={}",
           binary,
           args,
           protocol::LAUNCHER_PIPE_ENV,
           pipe);
    let child = command.stdout(Stdio::inherit())
                       .stderr(Stdio::inherit())
                       .env(protocol::LAUNCHER_PIPE_ENV, pipe)
                       .env(protocol::LAUNCHER_PID_ENV,
                            process::current_pid().to_string())
                       .args(args)
                       .spawn()
                       .context("Failed to spawn supervisor")?;
    Ok(child)
}

/// Determines the most viable Supervisor binary to run and returns a `PathBuf` to it.
///
/// Setting a filepath value to the `HAB_SUP_BINARY` env variable will force that binary to be used
/// instead.
fn supervisor_cmd() -> Result<PathBuf> {
    if let Ok(command) = core::env::var(SUP_CMD_ENVVAR) {
        return Ok(PathBuf::from(command));
    }
    let ident = PackageIdent::from_str(SUP_PACKAGE_IDENT).unwrap();
    let fs_root_path = FS_ROOT_PATH.as_path();
    match PackageInstall::load_at_least(&ident, Some(fs_root_path)) {
        Ok(install) => {
            match core::fs::find_command_in_pkg(SUP_CMD, &install, fs_root_path) {
                Ok(Some(cmd)) => Ok(cmd),
                _ => {
                    Err(anyhow!("Failed to locate '{}' binary in supervisor \
                                 package",
                                SUP_CMD))
                }
            }
        }
        Err(_) => Err(anyhow!("Failed to locate supervisor package, {}", SUP_PACKAGE_IDENT)),
    }
}
