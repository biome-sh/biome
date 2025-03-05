// Implementation of `bio pkg info` command
use clap_v4 as clap;

use std::path::PathBuf;

use clap::{ArgAction,
           Parser};

use biome_core::crypto;

use biome_common::{cli::clap_validators::FileExistsValueParser,
                     ui::UI};

use crate::{command::pkg::info,
            error::Result as BioResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgInfoOptions {
    /// Output will be rendered in json. (Includes extended metadata)
    #[arg(name = "TO_JSON",
          short = 'j',
          long = "json",
          action = ArgAction::SetTrue)]
    json: bool,

    // TODO: Move to semantic PathBuf after CLAP-v2 support is removed kept due to Clap V2 quirk
    /// A path to a Biome Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[arg(name = "SOURCE", value_parser = FileExistsValueParser)]
    source: String,
}

impl PkgInfoOptions {
    pub(super) fn do_info(&self, ui: &mut UI) -> BioResult<()> {
        crypto::init()?;

        info::start(ui, &Into::<PathBuf>::into(self.source.clone()), self.json)
    }
}
