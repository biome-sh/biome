//! Defines the data that we provide users in their template files.
//!
//! The data structures in this module effectively serve as wrappers
//! or proxies for other Supervisor-internal data structures. They use
//! `Cow` types for flexibility; in the normal code flow, they will
//! just take references to existing data, keeping the memory
//! footprint low. For tests, however, they can be created directly
//! with test data, which means you don't need to instantiate a lot of
//! complex data structures just to verify their behavior.
//!
//! Using custom data type proxies like this allows us to decouple the
//! internal data structures from the needs of the templating
//! engine. Since the ultimate purpose of the rendering context is to
//! create a JSON object, we can specify our own custom `Serialize`
//! implementations, completely separate from any implementations on
//! the original data structures. This allows us to further decouple
//! things, giving us the ability to add new fields to the rendering
//! context at the serialization level (e.g., to add the same data
//! under a different name, introduce new views on existing data,
//! etc.), which finally gives us a safe mechanism by which to evolve
//! our rendering context.
//!
//! As such, know that any changes to the `Serialize` implementations
//! in this module will have an immediate and direct effect on the
//! data that is available in users' templates. Make changes with care
//! and deliberation.
//!
//! To help guard against this, the entire structure of the rendering
//! context is also defined in a JSON Schema document, which is used
//! in tests to validate everything.
//!
//! All proxy types and implementations are private, to emphasize
//! their focused and single-use purpose; they shouldn't be used for
//! anything else, and so, they _can't_ be used for anything else.

use crate::{census::{CensusGroup,
                     CensusMemberProxy,
                     CensusRing,
                     ElectionStatus},
            manager::Sys};
use biome_common::templating::{config::Cfg,
                                 package::{Env,
                                           Pkg}};
use biome_core::{package::{FullyQualifiedPackageIdent,
                             Identifiable,
                             PackageIdent},
                   service::{ServiceBind,
                             ServiceGroup}};
use serde::{ser::SerializeMap,
            Serialize,
            Serializer};
use std::{borrow::Cow,
          collections::BTreeMap,
          net::IpAddr,
          path::PathBuf,
          result};

type SvcMember<'a> = CensusMemberProxy<'a>;

/// The context of a rendering call, exposing information on the
/// currently-running Supervisor and service, its service group, and
/// groups it is bound to. The JSON serialization of this
/// structure is what is exposed to users in their templates.
///
/// NOTE: This public interface of this structure is defined by its
/// Serde `Serialize` implementation (and those of its members), so
/// change this with care.
///
/// User-facing documentation is available at
/// https://www.habitat.sh/docs/reference/#template-data; update that
/// as required.
#[derive(Clone, Debug, Serialize)]
pub struct RenderContext<'a> {
    sys:  SystemInfo<'a>,
    pkg:  Package<'a>,
    cfg:  Cow<'a, Cfg>,
    svc:  Svc<'a>,
    bind: Binds<'a>,
}

impl<'a> RenderContext<'a> {
    /// Create a RenderContext that wraps a number of internal data
    /// structures, safely and selectively exposing the data to users
    /// in their templates.
    ///
    /// Note that we wrap everything except the `Cfg`, to which we
    /// maintain a direct reference. The serialization logic for this
    /// is already complex, and exactly what we need. Because of the
    /// nature of `Cfg`s behavior, we should be safe relying on that
    /// implementation for the foreseeable future.
    pub fn new<T>(service_group: &ServiceGroup,
                  sys: &'a Sys,
                  pkg: &'a Pkg,
                  cfg: &'a Cfg,
                  census: &'a CensusRing,
                  bindings: T)
                  -> RenderContext<'a>
        where T: Iterator<Item = &'a ServiceBind>
    {
        let census_group = census.census_group_for(service_group)
                                 .expect("Census Group missing from list!");
        RenderContext { sys:  SystemInfo::from_sys(sys),
                        pkg:  Package::from_pkg(pkg),
                        cfg:  Cow::Borrowed(cfg),
                        svc:  Svc::new(census_group),
                        bind: Binds::new(bindings, census), }
    }

    // Exposed only for logging... can probably do this another way.
    pub fn service_group_name(&self) -> String { format!("{}", self.svc.service_group) }
}

////////////////////////////////////////////////////////////////////////
// PRIVATE CODE BELOW
////////////////////////////////////////////////////////////////////////

/// Templating proxy for a `manager::Sys` struct.
///
/// Exposed to users under the `sys` key. This section represents Supervisor system information
/// such as the currently running version, administration ports and addresses, and other
/// information specific to the running Supervisor.
#[derive(Clone, Debug, Serialize)]
struct SystemInfo<'a> {
    version:           Cow<'a, String>,
    member_id:         Cow<'a, String>,
    ip:                Cow<'a, IpAddr>,
    hostname:          Cow<'a, String>,
    gossip_ip:         Cow<'a, IpAddr>,
    gossip_port:       Cow<'a, u16>,
    http_gateway_ip:   Cow<'a, IpAddr>,
    http_gateway_port: Cow<'a, u16>,
    ctl_gateway_ip:    Cow<'a, IpAddr>,
    ctl_gateway_port:  Cow<'a, u16>,
    permanent:         Cow<'a, bool>,
}

impl<'a> SystemInfo<'a> {
    fn from_sys(sys: &'a Sys) -> Self {
        SystemInfo { version:           Cow::Borrowed(&sys.version),
                     member_id:         Cow::Borrowed(&sys.member_id),
                     ip:                Cow::Borrowed(&sys.ip),
                     hostname:          Cow::Borrowed(&sys.hostname),
                     gossip_ip:         Cow::Borrowed(&sys.gossip_ip),
                     gossip_port:       Cow::Borrowed(&sys.gossip_port),
                     http_gateway_ip:   Cow::Borrowed(&sys.http_gateway_ip),
                     http_gateway_port: Cow::Borrowed(&sys.http_gateway_port),
                     ctl_gateway_ip:    Cow::Borrowed(&sys.ctl_gateway_ip),
                     ctl_gateway_port:  Cow::Borrowed(&sys.ctl_gateway_port),
                     permanent:         Cow::Borrowed(&sys.permanent), }
    }
}

////////////////////////////////////////////////////////////////////////

/// Templating proxy fro a `manager::service::Pkg` struct.
///
/// Currently exposed to users under the `pkg` key.
#[derive(Clone, Debug)]
struct Package<'a> {
    ident:           Cow<'a, FullyQualifiedPackageIdent>,
    deps:            Cow<'a, Vec<PackageIdent>>,
    env:             Cow<'a, Env>,
    // TODO (CM): Ideally, this would be Vec<u16>, since they're ports.
    exposes:         Cow<'a, Vec<String>>,
    exports:         Cow<'a, BTreeMap<String, String>>,
    // TODO (CM): Maybe Path instead of Cow<'a PathBuf>?
    path:            Cow<'a, PathBuf>,
    svc_path:        Cow<'a, PathBuf>,
    svc_config_path: Cow<'a, PathBuf>,
    svc_data_path:   Cow<'a, PathBuf>,
    svc_files_path:  Cow<'a, PathBuf>,
    svc_static_path: Cow<'a, PathBuf>,
    svc_var_path:    Cow<'a, PathBuf>,
    svc_pid_file:    Cow<'a, PathBuf>,
    svc_run:         Cow<'a, PathBuf>,
    svc_user:        Cow<'a, String>,
    svc_group:       Cow<'a, String>,
}

impl<'a> Package<'a> {
    fn from_pkg(pkg: &'a Pkg) -> Self {
        Package { ident:           Cow::Borrowed(&pkg.ident),
                  deps:            Cow::Borrowed(&pkg.deps),
                  env:             Cow::Borrowed(&pkg.env),
                  exposes:         Cow::Borrowed(&pkg.exposes),
                  exports:         Cow::Borrowed(&pkg.exports),
                  path:            Cow::Borrowed(&pkg.path),
                  svc_path:        Cow::Borrowed(&pkg.svc_path),
                  svc_config_path: Cow::Borrowed(&pkg.svc_config_path),
                  svc_data_path:   Cow::Borrowed(&pkg.svc_data_path),
                  svc_files_path:  Cow::Borrowed(&pkg.svc_files_path),
                  svc_static_path: Cow::Borrowed(&pkg.svc_static_path),
                  svc_var_path:    Cow::Borrowed(&pkg.svc_var_path),
                  svc_pid_file:    Cow::Borrowed(&pkg.svc_pid_file),
                  svc_run:         Cow::Borrowed(&pkg.svc_run),
                  svc_user:        Cow::Borrowed(&pkg.svc_user),
                  svc_group:       Cow::Borrowed(&pkg.svc_group), }
    }
}

impl Serialize for Package<'_> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        // Explicitly focusing on JSON serialization, which does not
        // need a length hint (thus the `None`)
        let mut map = serializer.serialize_map(None)?;

        // This is really the only thing that we need to have a custom
        // `Serialize` implementation for. Alternatively, we could
        // wrap our ident in a proxy type that has its own Serialize
        // implementation, but I think we're going to have some other
        // changes in this serialization format soon (e.g., around
        // `deps` and `exposes`, and eventually storing a
        // FullyQualifiedPackageIdent here).
        map.serialize_entry("ident", &self.ident.to_string())?;

        // Break out the components of the identifier, for easy access
        // in templates
        map.serialize_entry("origin", &self.ident.origin())?;
        map.serialize_entry("name", &self.ident.name())?;
        map.serialize_entry("version", &self.ident.version())?;
        map.serialize_entry("release", &self.ident.release())?;

        map.serialize_entry("deps", &self.deps)?;
        map.serialize_entry("env", &self.env)?;

        map.serialize_entry("exposes", &self.exposes)?;
        map.serialize_entry("exports", &self.exports)?;
        map.serialize_entry("path", &self.path)?;

        map.serialize_entry("svc_path", &self.svc_path)?;
        map.serialize_entry("svc_config_path", &self.svc_config_path)?;
        map.serialize_entry("svc_data_path", &self.svc_data_path)?;
        map.serialize_entry("svc_files_path", &self.svc_files_path)?;
        map.serialize_entry("svc_static_path", &self.svc_static_path)?;
        map.serialize_entry("svc_var_path", &self.svc_var_path)?;
        map.serialize_entry("svc_pid_file", &self.svc_pid_file)?;
        map.serialize_entry("svc_run", &self.svc_run)?;
        map.serialize_entry("svc_user", &self.svc_user)?;
        map.serialize_entry("svc_group", &self.svc_group)?;

        map.end()
    }
}

///////////////////////////////////////////////////////////////////////

/// Templating proxy around a `census::CensusGroup`.
///
/// Currently exposed to users under the `svc` key.
#[derive(Clone, Debug)]
struct Svc<'a> {
    service_group:          Cow<'a, ServiceGroup>,
    election_status:        Cow<'a, ElectionStatus>,
    update_election_status: Cow<'a, ElectionStatus>,
    members:                Vec<SvcMember<'a>>,
    leader:                 Option<SvcMember<'a>>,
    update_leader:          Option<SvcMember<'a>>,
    me:                     SvcMember<'a>,
    first:                  SvcMember<'a>,
}

impl<'a> Svc<'a> {
    // TODO (CM): rename to from_census_group
    fn new(census_group: &'a CensusGroup) -> Self {
        Svc { service_group:          Cow::Borrowed(&census_group.service_group),
              election_status:        Cow::Borrowed(&census_group.election_status),
              update_election_status: Cow::Borrowed(&census_group.update_election_status),
              members:                census_group.active_members()
                                                             .map(SvcMember::new)
                                                             .collect(),
              me:                     census_group.me()
                                                             .map(SvcMember::new)
                                                             .expect("Missing 'me'"),
              leader:                 census_group.leader()
                                                             .map(SvcMember::new),
              update_leader:          census_group.update_leader()
                                                             .map(SvcMember::new),
              first:
                  select_first(census_group).expect("First should always be present \
                                                                on svc" /* i.e. `me` will
                                                                         * always be
                                                                         * here, and alive */), }
    }
}

impl Serialize for Svc<'_> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        // Explicitly focusing on JSON serialization, which does not
        // need a length hint (thus the `None`)
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("service", &self.service_group.service())?;
        map.serialize_entry("group", &self.service_group.group())?;
        map.serialize_entry("org", &self.service_group.org())?;
        map.serialize_entry("election_is_running",
                            &(self.election_status.as_ref()
                              == &ElectionStatus::ElectionInProgress))?;
        map.serialize_entry("election_is_no_quorum",
                            &(self.election_status.as_ref() == &ElectionStatus::ElectionNoQuorum))?;
        map.serialize_entry("election_is_finished",
                            &(self.election_status.as_ref() == &ElectionStatus::ElectionFinished))?;
        map.serialize_entry("update_election_is_running",
                            &(self.update_election_status.as_ref()
                              == &ElectionStatus::ElectionInProgress))?;
        map.serialize_entry("update_election_is_no_quorum",
                            &(self.update_election_status.as_ref()
                              == &ElectionStatus::ElectionNoQuorum))?;
        map.serialize_entry("update_election_is_finished",
                            &(self.update_election_status.as_ref()
                              == &ElectionStatus::ElectionFinished))?;

        map.serialize_entry("me", &self.me)?;
        map.serialize_entry("members", &self.members)?;
        map.serialize_entry("leader", &self.leader)?;
        map.serialize_entry("first", &self.first)?;
        map.serialize_entry("update_leader", &self.update_leader)?;

        map.end()
    }
}

////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Serialize)]
struct Binds<'a>(BTreeMap<String, BindGroup<'a>>);

impl<'a> Binds<'a> {
    fn new<T>(bindings: T, census: &'a CensusRing) -> Self
        where T: Iterator<Item = &'a ServiceBind>
    {
        let mut map = BTreeMap::default();
        for bind in bindings {
            if let Some(group) = census.census_group_for(bind.service_group()) {
                map.insert(bind.name().to_string(), BindGroup::new(group));
            }
        }
        Binds(map)
    }
}

#[derive(Clone, Debug, Serialize)]
struct BindGroup<'a> {
    first:   Option<SvcMember<'a>>,
    leader:  Option<SvcMember<'a>>,
    members: Vec<SvcMember<'a>>,
}

impl<'a> BindGroup<'a> {
    fn new(group: &'a CensusGroup) -> Self {
        BindGroup { first:   select_first(group),
                    leader:  group.leader().map(SvcMember::new),
                    members: group.active_members().map(SvcMember::new).collect(), }
    }
}

////////////////////////////////////////////////////////////////////////

/// Helper for pulling the leader or first member from a census
/// group. This is used to populate the `.first` field in `bind` and
/// `svc`.
///
/// IMPORTANT
///
/// The `first` field is now deprecated; in order to not change its
/// behavior until we remove it altogether, we'll continue to populate
/// it from *all* members, and not just active members. Users should
/// move away from using `first`, and should instead just use
/// `members[0]`, or `leader`.
fn select_first(census_group: &CensusGroup) -> Option<SvcMember<'_>> {
    match census_group.leader() {
        Some(member) => Some(SvcMember::new(member)),
        None => census_group.members().next().map(SvcMember::new),
    }
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{census::CensusMember,
                manager::service::Cfg,
                test_helpers::*};
    use biome_butterfly::rumor::service::SysInfo;
    use biome_common::templating::{config::PackageConfigPaths,
                                     TemplateRenderer};
    use biome_core::package::PackageIdent;
    use std::{fs,
              io::{Read,
                   Write},
              net::{IpAddr,
                    Ipv4Addr},
              path::PathBuf};
    use tempfile::TempDir;

    ////////////////////////////////////////////////////////////////////////

    // These structs, functions, and impls are copied from
    // manager::service::config::test, and are used to create a
    // suitable `Cfg` struct for these tests.

    struct TestPkg {
        base_path: PathBuf,
    }

    impl TestPkg {
        fn new(tmp: &TempDir) -> Self {
            let pkg = Self { base_path: tmp.path().to_owned(), };

            fs::create_dir_all(pkg.default_config_dir()).expect("create deprecated user config \
                                                                 dir");
            fs::create_dir_all(pkg.recommended_user_config_dir()).expect("create recommended \
                                                                          user config dir");
            fs::create_dir_all(pkg.deprecated_user_config_dir()).expect("create default config \
                                                                         dir");
            pkg
        }
    }

    impl PackageConfigPaths for TestPkg {
        fn name(&self) -> String { String::from("testing") }

        fn default_config_dir(&self) -> PathBuf { self.base_path.join("root") }

        fn recommended_user_config_dir(&self) -> PathBuf { self.base_path.join("user") }

        fn deprecated_user_config_dir(&self) -> PathBuf { self.base_path.join("svc") }
    }

    fn new_test_pkg() -> (TempDir, TestPkg) {
        let tmp = TempDir::new().expect("create temp dir");
        let pkg = TestPkg::new(&tmp);

        let default_toml = pkg.default_config_dir().join("default.toml");
        let mut buffer = fs::File::create(default_toml).expect("couldn't write file");
        buffer.write_all(
                         br#"
foo = "bar"
baz = "boo"

[foobar]
one = 1
two = 2
"#,
        )
              .expect("Couldn't write default.toml");
        (tmp, pkg)
    }

    ////////////////////////////////////////////////////////////////////////

    /// Create a basic SvcMember struct for use in tests
    fn default_svc_member<'a>() -> SvcMember<'a> {
        let ident = PackageIdent::new("core", "test_pkg", Some("1.0.0"), Some("20180321150416"));
        let census_member = CensusMember { member_id: "MEMBER_ID".into(),
                                           pkg: ident,
                                           pkg_incarnation: 0,
                                           service: "foo".into(),
                                           group: "default".into(),
                                           org: None,
                                           persistent: true,
                                           leader: false,
                                           follower: false,
                                           update_leader: false,
                                           update_follower: false,
                                           election_is_running: false,
                                           election_is_no_quorum: false,
                                           election_is_finished: false,
                                           update_election_is_running: false,
                                           update_election_is_no_quorum: false,
                                           update_election_is_finished: false,
                                           sys: SysInfo::default(),
                                           alive: true,
                                           suspect: false,
                                           confirmed: false,
                                           departed: false,
                                           cfg: toml::value::Table::new(), };
        SvcMember::new_owned(census_member)
    }

    /// Just create a basic RenderContext that could be used in tests.
    ///
    /// If you want to modify parts of it, it's easier to change
    /// things on a mutable reference.
    fn default_render_context<'a>() -> RenderContext<'a> {
        let system_info =
            SystemInfo { version:           Cow::Owned("I AM A BIOME VERSION".into()),
                         member_id:         Cow::Owned("MEMBER_ID".into()),
                         ip:                Cow::Owned(IpAddr::V4(Ipv4Addr::LOCALHOST)),
                         hostname:          Cow::Owned("MY_HOSTNAME".into()),
                         gossip_ip:         Cow::Owned(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
                         gossip_port:       Cow::Owned(1234),
                         http_gateway_ip:   Cow::Owned(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
                         http_gateway_port: Cow::Owned(5678),
                         ctl_gateway_ip:    Cow::Owned(IpAddr::V4(Ipv4Addr::LOCALHOST)),
                         ctl_gateway_port:  Cow::Owned(5679),
                         permanent:         Cow::Owned(false), };

        let ident = FullyQualifiedPackageIdent::new("core", "test_pkg", "1.0.0", "20180321150416");

        let deps = vec![PackageIdent::new("test", "pkg1", Some("1.0.0"), Some("20180321150416")),
                        PackageIdent::new("test", "pkg2", Some("2.0.0"), Some("20180321150416")),
                        PackageIdent::new("test", "pkg3", Some("3.0.0"), Some("20180321150416")),];

        let mut env_hash = BTreeMap::new();
        env_hash.insert("PATH".into(), "/foo:/bar:/baz".into());
        env_hash.insert("SECRET".into(), "sooperseekrit".into());

        let mut export_hash = BTreeMap::new();
        export_hash.insert("blah".into(), "stuff.thing".into());
        export_hash.insert("port".into(), "test_port".into());

        let pkg = Package { ident:           Cow::Owned(ident.clone()),
                            deps:            Cow::Owned(deps),
                            env:             Cow::Owned(env_hash.into()),
                            exposes:         Cow::Owned(vec!["1234".into(),
                                                             "8000".into(),
                                                             "2112".into()]),
                            exports:         Cow::Owned(export_hash),
                            path:            Cow::Owned("my_path".into()),
                            svc_path:        Cow::Owned("svc_path".into()),
                            svc_config_path: Cow::Owned("config_path".into()),
                            svc_data_path:   Cow::Owned("data_path".into()),
                            svc_files_path:  Cow::Owned("files_path".into()),
                            svc_static_path: Cow::Owned("static_path".into()),
                            svc_var_path:    Cow::Owned("var_path".into()),
                            svc_pid_file:    Cow::Owned("pid_file".into()),
                            svc_run:         Cow::Owned("svc_run".into()),
                            svc_user:        Cow::Owned("hab".into()),
                            svc_group:       Cow::Owned("hab".into()), };

        let group: ServiceGroup = "foo.default".parse().unwrap();

        // Not using _tmp_dir, but need it to prevent it from being
        // dropped before we make the Cfg
        let (_tmp_dir, test_pkg) = new_test_pkg();
        let cfg = Cfg::new(&test_pkg, None).expect("create config");

        // TODO (CM): just create a toml table directly
        let mut svc_member_cfg = toml::value::Table::new();
        svc_member_cfg.insert("foo".into(), "bar".into());

        let mut me = default_svc_member();
        let me_mut = me.to_mut();
        me_mut.pkg = ident.into();
        me_mut.cfg = svc_member_cfg;

        let svc = Svc { service_group:          Cow::Owned(group),
                        election_status:        Cow::Owned(ElectionStatus::ElectionInProgress),
                        update_election_status: Cow::Owned(ElectionStatus::ElectionFinished),
                        members:                vec![me.clone()],
                        leader:                 None,
                        update_leader:          None,
                        me:                     me.clone(),
                        first:                  me.clone(), };

        let mut bind_map = BTreeMap::new();
        let bind_group = BindGroup { first:   Some(me.clone()),
                                     leader:  None,
                                     members: vec![me.clone()], };
        bind_map.insert("foo".into(), bind_group);
        let binds = Binds(bind_map);

        RenderContext { sys: system_info,
                        pkg,
                        cfg: Cow::Owned(cfg),
                        svc,
                        bind: binds }
    }

    /// Render the given template string using the given context,
    /// returning the result. This can help to verify that
    /// RenderContext data are accessible to users in the way we
    /// expect.
    fn render(template_content: &str, ctx: &RenderContext<'_>) -> String {
        let mut renderer = TemplateRenderer::new();
        renderer.register_template_string("testing", template_content)
                .expect("Could not register template content");
        renderer.render("testing", ctx)
                .expect("Could not render template")
    }

    ////////////////////////////////////////////////////////////////////////

    /// Reads a file containing real rendering context output from an
    /// actual Supervisor, prior to the refactoring to separate the
    /// serialization logic from the internal data structures.
    #[test]
    fn sample_context_is_valid() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
                                                            .join("fixtures")
                                                            .join("sample_render_context.json");

        let mut f = fs::File::open(path).expect("could not open sample_render_context.json");
        let mut json = String::new();
        f.read_to_string(&mut json)
         .expect("could not read sample_render_context.json");

        assert_valid(&json, "render_context_schema.json");
    }

    #[test]
    fn trivial_failure() {
        let state = validate_string(r#"{"svc":{},"pkg":{},"cfg":{},"svc":{},"bind":{}}"#,
                                    "render_context_schema.json");
        assert!(!state.is_valid(),
                "Expected schema validation to fail, but it succeeded!");
    }

    #[test]
    fn default_render_context_is_valid() {
        let render_context = default_render_context();
        let j = serde_json::to_string(&render_context).expect("can't serialize to JSON");
        assert_valid(&j, "render_context_schema.json");
    }

    // This mainly exists to show how you could modify the default
    // context for easily testing different scenarios.
    #[test]
    fn no_binds_are_valid() {
        let mut render_context = default_render_context();
        render_context.bind = Binds(BTreeMap::new());
        let j = serde_json::to_string(&render_context).expect("can't serialize to JSON");
        assert_valid(&j, "render_context_schema.json");
    }

    #[test]
    fn binds_are_output_in_consistent_order() {
        let mut render_context = default_render_context();
        let mut new_binds = BTreeMap::new();

        new_binds.insert("foo".to_string(),
                         BindGroup { leader:  None,
                                     first:   None,
                                     members: vec![], });
        new_binds.insert("bar".to_string(),
                         BindGroup { leader:  None,
                                     first:   None,
                                     members: vec![], });
        new_binds.insert("quux".to_string(),
                         BindGroup { leader:  None,
                                     first:   None,
                                     members: vec![], });
        new_binds.insert("baz".to_string(),
                         BindGroup { leader:  None,
                                     first:   None,
                                     members: vec![], });

        render_context.bind = Binds(new_binds);

        let j = serde_json::to_string(&render_context).expect("can't serialize to JSON");
        assert_valid(&j, "render_context_schema.json");

        // The point here is to just render our context to a string repeatedly and compare it to
        // our original rendered string. Our goal was to have consistent ordering of binds on every
        // render and this should be sufficient to tell us if that's the case.
        for _ in 0..50 {
            let x = serde_json::to_string(&render_context).expect("can't serialize to JSON");
            assert_eq!(j, x);
        }
    }

    #[test]
    fn no_leader_renders_correctly() {
        let ctx = default_render_context();

        // Just make sure our default context is set up how this test
        // is expecting
        assert!(ctx.bind.0["foo"].leader.is_none());

        let output = render("{{#if bind.foo.leader}}THERE IS A LEADER{{else}}NO LEADER{{/if}}",
                            &ctx);

        assert_eq!(output, "NO LEADER");
    }

    #[test]
    fn leader_renders_correctly() {
        let mut ctx = default_render_context();

        // Let's create a new leader, with a custom member_id
        let mut svc_member = default_svc_member();
        svc_member.to_mut().member_id = "samshamandthepharaohs".into();

        // Set up our own bind with a leader
        let mut bind_map = BTreeMap::new();
        let bind_group = BindGroup { first:   Some(svc_member.clone()),
                                     leader:  Some(svc_member.clone()),
                                     members: vec![svc_member.clone()], };
        bind_map.insert("foo".into(), bind_group);
        let binds = Binds(bind_map);
        ctx.bind = binds;

        // This template should reveal the member_id of the leader
        let output = render("{{#if bind.foo.leader}}{{bind.foo.leader.member_id}}{{else}}NO \
                             LEADER{{/if}}",
                            &ctx);

        assert_eq!(output, "samshamandthepharaohs");
    }

    // Technically, `bind.<SERVICE>.first` could be None, according to
    // the typing of the code.  This was always been technically
    // possible, even though for practical purposes, it will be
    // Some. This test just confirms that the JSON schema is
    // technically in line with the Rust code, until we are able to
    // remove `first` altogether.
    //
    // `svc.first` can't be `None` currently, because that would mean
    // that the current Supervisor doesn't know about itself.
    #[test]
    fn bind_first_can_technically_be_none() {
        let mut render_context = default_render_context();
        let mut new_binds = BTreeMap::new();

        new_binds.insert("foo".to_string(),
                         BindGroup { leader:  None,
                                     first:   None,
                                     members: vec![], });

        render_context.bind = Binds(new_binds);
        let j = serde_json::to_string(&render_context).expect("can't serialize to JSON");
        assert_valid(&j, "render_context_schema.json");
    }
}
