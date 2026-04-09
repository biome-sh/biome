use crate::error::Result as BioResult;
use biome_common::ui::UI;
use clap::Subcommand;
use clap_v4 as clap;

mod generate;
use generate::UserKeyGenerateOptions;

#[derive(Debug, Clone, Subcommand)]
#[command(
    rename_all = "kebab-case",
    arg_required_else_help = true,
    about = "Commands relating to Biome user keys",
    help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n"
)]
pub(crate) enum UserKeyCommand {
    /// Generates a Biome user key
    Generate(UserKeyGenerateOptions),
}

impl UserKeyCommand {
    pub(crate) async fn do_key(&self, ui: &mut UI) -> BioResult<()> {
        match self {
            UserKeyCommand::Generate(opts) => opts.do_generate(ui).await,
        }
    }
}
