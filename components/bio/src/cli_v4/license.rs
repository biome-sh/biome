use clap_v4 as clap;

use crate::{error::Result as BioResult, license};
use biome_common::ui::UI;
use clap::Subcommand;

#[derive(Clone, Debug, Subcommand)]
#[command(
    author = "\nThe Biome Maintainers <humans@biome.sh>",
    about = "Commands relating to Biome license agreements",
    arg_required_else_help = true,
    help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n"
)]
pub(super) enum LicenseCommand {
    /// Accept the Biome Binary Distribution Agreement without prompting
    Accept,
}

impl LicenseCommand {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> BioResult<()> {
        match self {
            Self::Accept => Ok(()),
        }
    }
}
