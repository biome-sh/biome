use crate::error::Result as BioResult;
use biome_common::ui::UI;
use clap::Subcommand;
use clap_v4 as clap;

mod key;
use key::UserKeyCommand;

#[derive(Debug, Clone, Subcommand)]
#[command(
    rename_all = "kebab-case",
    arg_required_else_help = true,
    about = "Commands relating to Biome users",
    help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n"
)]
pub(crate) enum UserCommand {
    /// Commands relating to Biome user keys
    #[command(subcommand)]
    Key(UserKeyCommand),
}

impl UserCommand {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> BioResult<()> {
        match self {
            UserCommand::Key(cmd) => cmd.do_key(ui).await,
        }
    }
}
