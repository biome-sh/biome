// Implemenatation of `bio pkg binlink`

use clap_v4 as clap;

use std::path::PathBuf;

use clap::{ArgAction,
           Parser};

use biome_core::{fs::FS_ROOT_PATH,
                   package::PackageIdent};

use biome_common::{cli::{clap_validators::BioPkgIdentValueParser,
                           BINLINK_DIR_ENVVAR,
                           DEFAULT_BINLINK_DIR},
                     ui::UI};

use crate::{command::pkg::binlink,
            error::Result as BioResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgBinlinkOptions {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT", value_parser = BioPkgIdentValueParser::simple())]
    pkg_ident: PackageIdent,

    /// The command to binlink (ex: bash)
    #[arg(name = "BINARY")]
    binary: Option<String>,

    /// Set the destination directory
    #[arg(name = "DEST_DIR",
        short = 'd',
        long = "dest",
        env = BINLINK_DIR_ENVVAR,
        default_value = DEFAULT_BINLINK_DIR)]
    dest_dir: PathBuf,

    /// Overwrite existing binlinks
    #[arg(name = "FORCE", short = 'f', long = "force", action = ArgAction::SetTrue)]
    force: bool,
}

impl PkgBinlinkOptions {
    pub(super) fn do_binlink(&self, ui: &mut UI) -> BioResult<()> {
        if let Some(binary) = &self.binary {
            binlink::start(ui,
                           &self.pkg_ident,
                           &binary,
                           &self.dest_dir,
                           &FS_ROOT_PATH,
                           self.force)
        } else {
            binlink::binlink_all_in_pkg(ui,
                                        &self.pkg_ident,
                                        &self.dest_dir,
                                        &FS_ROOT_PATH,
                                        self.force)
        }
    }
}
