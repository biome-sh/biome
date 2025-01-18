// Implementation of `bio pkg provides` command

use clap_v4 as clap;

use clap::{ArgAction,
           Parser};

use biome_core::fs::FS_ROOT_PATH;

use crate::{command::pkg::provides,
            error::Result as BioResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgProvidesOptions {
    /// File name to find
    #[arg(name = "FILE")]
    file: String,

    /// Show fully qualified package names (ex: core/busybox-static/1.24.2/20160708162350)
    #[arg(name = "FULL_RELEASES", short = 'r', action = ArgAction::SetTrue)]
    full_releases: bool,

    /// Show full path to file
    #[arg(name = "FULL_PATHS", short = 'p', action = ArgAction::SetTrue)]
    full_paths: bool,
}

impl PkgProvidesOptions {
    pub(super) fn do_provides(&self) -> BioResult<()> {
        provides::start(&self.file,
                        &FS_ROOT_PATH,
                        self.full_releases,
                        self.full_paths)
    }
}
