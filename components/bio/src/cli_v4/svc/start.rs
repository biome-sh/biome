use clap_v4 as clap;

use clap::Parser;

use biome_common::cli::clap_validators::BioPkgIdentValueParser;
use biome_core::package::PackageIdent;

use crate::{cli_v4::utils::RemoteSup, error::Result as BioResult, gateway_util};

/// Start a loaded, but stopped, Biome service.
#[derive(Clone, Debug, Parser)]
#[command(
    author = "\nThe Biome Maintainers <humans@biome.sh>",
    help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n"
)]
pub(crate) struct StartCommand {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT", value_parser = BioPkgIdentValueParser::simple())]
    pkg_ident: PackageIdent,

    #[command(flatten)]
    remote_sup: RemoteSup,
}

impl StartCommand {
    pub(crate) async fn do_command(&self) -> BioResult<()> {
        let remote_sup = self.remote_sup.clone();
        let msg = biome_sup_protocol::ctl::SvcStart {
            ident: Some(self.pkg_ident.clone().into()),
        };
        gateway_util::send(remote_sup.inner(), msg).await
    }
}
