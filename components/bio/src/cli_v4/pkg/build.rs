// Implemenatation of `bio pkg build`
use clap_v4 as clap;

use std::path::PathBuf;

use clap::{ArgAction,
           Parser};

use biome_common::ui::UI;

use biome_common::FeatureFlag;

use biome_core::{crypto,
                   crypto::keys::KeyCache,
                   origin::Origin};

use crate::{command::pkg::build,
            error::Result as BioResult};

#[cfg(target_os = "linux")]
use crate::error::Error as BioError;

use crate::cli_v4::utils::CacheKeyPath;

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgBuildOptions {
    // TODO: Should multiple Origins be supported? The semantics looks like that but the original
    // v2 code does not look like supporting.
    /// Installs secret origin keys (ex: "unicorn", "acme,other,acme-ops")
    #[arg(name = "HAB_ORIGIN_KEYS", short = 'k', long = "keys", action = ArgAction::Append)]
    bio_origin_keys: Vec<Origin>,

    // TODO: Make it a more semantic `PathBuf` Currently not done due to limitation of
    // `command::pkg::build`. Revisit it after removing `clap-v2`
    /// Sets the Studio root (default: /hab/studios/<DIR_NAME>)
    #[arg(name = "HAB_STUDIO_ROOT", short = 'r', long = "root")]
    bio_studio_root: Option<String>,

    // TODO: Same as above
    /// Sets the source path [default: $PWD]
    #[arg(name = "SRC_PATH", short = 's', long = "src")]
    src_path: Option<String>,

    // TODO : Same as above
    /// A directory containing a plan file or a `habitat/` directory which contains the plan
    /// file
    #[arg(name = "PLAN_CONTEXT")]
    plan_context: String,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,

    #[cfg(target_os = "linux")]
    /// Build a native package on the host system without a studio
    #[arg(name = "NATIVE_PACKAGE", short = 'N', long = "native-package", conflicts_with_all = &["REUSE", "DOCKER"])]
    native_package: bool,

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    /// Reuses a previous Studio for the build (default: clean up before building)
    // Only a truly native/local Studio can be reused--the Docker implementation will always be
    // ephemeral
    #[arg(name = "REUSE", short = 'R', long = "reuse", action = ArgAction::SetTrue)]
    reuse: bool,

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    /// Uses a Dockerized Studio for the build
    #[arg(name = "DOCKER", short = 'D', long = "docker", action = ArgAction::SetTrue)]
    docker: bool,

    /// Channel used to retrieve plan dependencies for Chef supported origins
    #[arg(name = "REFRESH_CHANNEL",
          short = 'f',
          long = "refresh-channel",
          env = "HAB_REFRESH_CHANNEL",
          default_value = "stable")]
    refresh_channel: Option<String>,
}

impl PkgBuildOptions {
    // Required because of lot of `cfg`...
    #[allow(unused_variables)]
    pub(super) async fn do_build(&self, ui: &mut UI, feature_flags: FeatureFlag) -> BioResult<()> {
        if !self.bio_origin_keys.is_empty() {
            crypto::init()?;
            let key_cache = KeyCache::new::<PathBuf>((&self.cache_key_path).into());
            for origin in self.bio_origin_keys.iter() {
                // Validate that a secret signing key is present on disk
                // for each origin.
                key_cache.latest_secret_origin_signing_key(origin)?;
            }
        }

        let native_package = false;

        let native_package = self.should_build_native_package(feature_flags)?;

        let (reuse_flag, docker_flag) = (false, false);

        #[cfg(any(target_os = "linux", target_os = "windows"))]
        let (reuse_flag, docker_flag) = (self.reuse, self.docker);

        build::start(ui,
                     self.plan_context.as_ref(),
                     self.bio_studio_root.as_deref(),
                     self.src_path.as_deref(),
                     &self.bio_origin_keys,
                     native_package,
                     reuse_flag,
                     docker_flag,
                     self.refresh_channel.as_deref()).await
    }

    #[cfg(target_os = "linux")]
    fn should_build_native_package(&self, feature_flags: FeatureFlag) -> BioResult<bool> {
        if self.native_package {
            if !feature_flags.contains(FeatureFlag::NATIVE_PACKAGE_SUPPORT) {
                return Err(BioError::ArgumentError(String::from("`--native-package` is only \
                                                                 available when \
                                                                 `HAB_FEAT_NATIVE_PACKAGE_SUPPORT` \
                                                                 is set")));
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn should_build_native_package(&self, _: FeatureFlag) -> BioResult<bool> { Ok(false) }
}
