// Implemenatation of `bio pkg config`

use clap_v4 as clap;

use clap::Parser;

use biome_core::{fs::FS_ROOT_PATH,
                   package::PackageIdent};

use biome_common::{cli::clap_validators::BioPkgIdentValueParser,
                     command::package::config};

use crate::error::Result as BioResult;

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgConfigOptions {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT", value_parser = BioPkgIdentValueParser::simple())]
    pkg_ident: PackageIdent,
}

impl PkgConfigOptions {
    pub(super) fn do_config(&self) -> BioResult<()> {
        config::start(&self.pkg_ident, &*FS_ROOT_PATH).map_err(Into::into)
    }
}
