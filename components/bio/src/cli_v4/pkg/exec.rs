// Implementation of `bio pkg exec` command

use clap_v4 as clap;

use std::{ffi::OsString,
          path::PathBuf};

use clap::Parser;

use biome_common::cli::clap_validators::BioPkgIdentValueParser;

use biome_core::package::PackageIdent;

use crate::{command::pkg::exec,
            error::Result as BioResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgExecOptions {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT", value_parser = BioPkgIdentValueParser::simple())]
    pkg_ident: PackageIdent,

    /// The command to execute (ex: ls)
    #[arg(name = "CMD")]
    cmd: PathBuf,

    /// Arguments to be passed to the command
    #[arg(name = "ARGS")]
    args: Vec<String>,
}

impl PkgExecOptions {
    pub(super) fn do_exec(&self) -> BioResult<()> {
        // Required to convert to OsStr
        // TODO: This should be internal implementation detail later on and move to actual command
        // implementation when `v2` is removed
        let args = self.args.iter().map(Into::into).collect::<Vec<OsString>>();
        exec::start(&self.pkg_ident, &self.cmd, &args)
    }
}
