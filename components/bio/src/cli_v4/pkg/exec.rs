// Implementation of `bio pkg exec` command

use clap_v4 as clap;

use clap::Parser;

use biome_common::cli::clap_validators::BioPkgIdentValueParser;

use biome_core::package::PackageIdent;

use crate::{cli_v4::utils::CommandAndArgs, command::pkg::exec, error::Result as BioResult};

#[derive(Debug, Clone, Parser)]
#[command(
    arg_required_else_help = true,
    help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n"
)]
pub(crate) struct PkgExecOptions {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT", value_parser = BioPkgIdentValueParser::simple())]
    pkg_ident: PackageIdent,

    #[command(flatten)]
    cmd: CommandAndArgs,
}

impl PkgExecOptions {
    pub(super) fn do_exec(&self) -> BioResult<()> {
        exec::start(&self.pkg_ident, &self.cmd.cmd, &self.cmd.args)
    }
}
