use crate::{
    cli_v4::utils::{AuthToken, origin_param_or_env},
    command::bldr::channel::demote::start,
    error::Result as BioResult,
};
use biome_common::ui::UI;
use biome_core::{ChannelIdent, origin::Origin};
use clap::Parser;
use clap_v4 as clap;
#[derive(Debug, Clone, Parser)]
#[command(
    arg_required_else_help = true,
    help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n"
)]
pub(crate) struct DemoteOpts {
    /// The channel from which all packages will be selected for demotion
    #[arg(value_name = "SOURCE_CHANNEL", value_parser = clap::value_parser!(ChannelIdent))]
    source_channel: ChannelIdent,

    /// The channel selected packages will be removed from
    #[arg(value_name = "TARGET_CHANNEL", value_parser = clap::value_parser!(ChannelIdent))]
    target_channel: ChannelIdent,

    /// Authentication token for Builder [env: BIO_AUTH_TOKEN]
    #[command(flatten)]
    token: AuthToken,

    /// Specify an alternate Builder endpoint [env: BIO_BLDR_URL] [default: https://bldr.biome.sh]
    #[arg(
        short = 'u',
        long = "url",
        value_name = "BLDR_URL",
        env = "BIO_BLDR_URL",
        default_value = "https://bldr.biome.sh"
    )]
    url: String,

    /// Sets the origin to which the channel belongs. Default is from 'BIO_ORIGIN' or cli.toml
    #[arg(short = 'o', long, value_name = "ORIGIN", value_parser = clap::value_parser!(Origin))]
    origin: Option<Origin>,
}

impl DemoteOpts {
    pub(crate) async fn do_demote(&self, ui: &mut UI) -> BioResult<()> {
        let origin = origin_param_or_env(&self.origin)?;
        let token = self.token.from_cli_or_config()?;
        start(
            ui,
            &self.url,
            &token,
            &origin,
            &self.source_channel,
            &self.target_channel,
        )
        .await
    }
}
