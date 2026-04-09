use crate::{common::ui::UI, error::Result};
use std::ffi::OsString;

const EXPORT_CMD_ENVVAR: &str = "BIO_PKG_EXPORT_TAR_BINARY";
const EXPORT_PKG_IDENT_ENVVAR: &str = "BIO_PKG_EXPORT_TAR_PKG_IDENT";
const EXPORT_CMD: &str = "bio-pkg-export-tar";

pub async fn start(ui: &mut UI, args: &[OsString]) -> Result<()> {
    crate::command::pkg::export::export_common::start(
        ui,
        args,
        EXPORT_CMD_ENVVAR,
        EXPORT_PKG_IDENT_ENVVAR,
        EXPORT_CMD,
    )
    .await
}
