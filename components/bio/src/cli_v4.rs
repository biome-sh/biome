use clap_v4 as clap;

use clap::Parser;

use biome_common::{FeatureFlag, ui::UI};

use crate::command::sup::start;

use crate::{VERSION, error::Result as BioResult};

use crate::license::check_for_license_acceptance_and_prompt;

mod bldr;
use bldr::BldrCommand;

mod cli;
use cli::CliCommand;

mod config;
use config::ConfigCommand;

mod file;
use file::FileCommand;

mod pkg;
use pkg::PkgCommand;

mod ring;
use ring::RingCommand;

mod user;
use user::UserCommand;

mod supportbundle;
use supportbundle::SupportBundleOpts;

pub(crate) mod sup;
use sup::SupCommand;

mod origin;
use origin::OriginCommand;

mod svc;
use svc::SvcCommand;

pub(crate) mod utils;

mod license;
use license::LicenseCommand;

mod studio;
use studio::StudioOpts;

mod plan;
use plan::PlanCommand;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Parser)]
#[command(name = "bio",
            version = VERSION,
            about = "🌱 Biome — the natural habitat for your services\n🔥 Because it works on my machine. And on yours. And in prod",
            author = "\nThe Biome Maintainers <humans@biome.sh>",
            arg_required_else_help = true,
            propagate_version = true,
            term_width = 100,
            help_template = "{name} {version} {author-section} {about-section} \
                    \n{usage-heading} {usage}\n\n{all-args}\n",
        )]
enum Bio {
    /// Commands relating to Biome Builder
    #[clap(subcommand)]
    Bldr(BldrCommand),

    /// Commands relating to Biome runtime config
    #[clap(subcommand)]
    Cli(CliCommand),

    /// Commands relating to a Service's runtime config
    #[clap(subcommand)]
    Config(ConfigCommand),

    /// Commands relating to Biome files
    #[clap(subcommand)]
    File(FileCommand),

    /// Commands relating to Biome license agreements
    #[clap(subcommand)]
    License(LicenseCommand),

    /// Commands relating to Biome Builder origins
    #[clap(subcommand)]
    Origin(OriginCommand),

    /// Commands relating to Biome packages
    #[clap(subcommand)]
    Pkg(PkgCommand),

    /// Commands relating to plans and other app-specific configuration
    #[clap(subcommand)]
    Plan(PlanCommand),

    /// Commands relating to Biome rings
    #[clap(subcommand)]
    Ring(RingCommand),

    /// Commands relating to Biome Studios
    #[cfg(any(
        target_os = "macos",
        any(
            all(target_os = "linux", any(target_arch = "x86_64", target_arch = "aarch64")),
            all(target_os = "windows", target_arch = "x86_64")
        )
    ))]
    #[command(name = "studio")]
    Studio(StudioOpts),

    /// The Biome Supervisor
    #[clap(subcommand)]
    Sup(SupCommand),

    /// Create a tarball of Biome Supervisor data to send to support
    #[command(name = "supportbundle")]
    SupportBundle(SupportBundleOpts),

    /// Commands relating to Biome Services
    #[clap(subcommand)]
    Svc(SvcCommand),

    /// Commands relating to Biome users
    #[clap(subcommand)]
    User(UserCommand),
}

impl Bio {
    async fn do_cli_command(&self, ui: &mut UI, feature_flags: FeatureFlag) -> BioResult<()> {
        match self {
            Self::Pkg(pkg_command) => pkg_command.do_command(ui, feature_flags).await,
            Self::Sup(sup_command) => sup_command.do_command(ui, feature_flags).await,
            Self::Origin(origin_command) => origin_command.do_command(ui, feature_flags).await,
            Self::Config(config_command) => config_command.do_command(ui).await,
            Self::File(file_command) => file_command.do_command(ui).await,
            Self::User(user_command) => user_command.do_command(ui).await,
            Self::Svc(svc_command) => svc_command.do_command(ui, feature_flags).await,
            Self::License(license_command) => license_command.do_command(ui).await,
            Self::Cli(cli_command) => cli_command.do_command(ui, feature_flags).await,
            Self::Bldr(bldr_command) => bldr_command.do_command(ui).await,
            Self::Ring(ring_command) => ring_command.do_command(ui).await,
            Self::Studio(studio_command) => studio_command.do_command(ui).await,
            Self::Plan(plan_command) => plan_command.do_command(ui).await,
            Self::SupportBundle(support_bundle_command) => support_bundle_command.do_command(ui).await,
        }
    }
}

#[derive(Clone, Debug, Parser)]
pub(crate) struct ServiceConfigCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct PkgInstallCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SupRunCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SvcStartCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SvcStopCommand;

pub async fn cli_driver(ui: &mut UI, feature_flags: FeatureFlag) -> BioResult<()> {
    let args: Vec<String> = std::env::args().collect();

    // We must manually detect a supervisor version check and call the `bio-sup` binary to get the
    // true Supervisor version.
    if args.len() >= 3
        && args.get(1).is_some_and(|arg| arg == "sup")
        && args.get(2).is_some_and(|arg| arg == "--version" || arg == "-V")
    {
        let os_args: Vec<std::ffi::OsString> = std::env::args_os().skip(2).collect();
        return start(ui, &os_args).await;
    }

    // Skip license check if user is accepting the license or just asking for help or version
    let skip_license_check = args.get(1).is_some_and(|arg| arg == "license")
        || args
            .iter()
            .any(|arg| arg == "--help" || arg == "-h" || arg == "--version" || arg == "-V");

    if !skip_license_check {
        check_for_license_acceptance_and_prompt(ui)?;
    }

    let cli = Bio::parse();
    cli.do_cli_command(ui, feature_flags).await
}
