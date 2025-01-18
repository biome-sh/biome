// Implementation of `bio pkg env` command

use clap_v4 as clap;

use clap::Parser;

use biome_common::cli::clap_validators::BioPkgIdentValueParser;

use biome_core::{fs::FS_ROOT_PATH,
                   package::PackageIdent};

use crate::command::pkg::env;

use crate::error::Result as BioResult;

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgEnvOptions {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT", value_parser = BioPkgIdentValueParser::simple())]
    pkg_ident: PackageIdent,
}

impl PkgEnvOptions {
    pub(super) fn do_env(&self) -> BioResult<()> {
        env::start(&self.pkg_ident, &*FS_ROOT_PATH).map_err(Into::into)
    }
}
