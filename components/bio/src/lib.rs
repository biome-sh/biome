#![recursion_limit = "128"]

use biome_api_client as api_client;
use biome_common as common;
use biome_core as hcore;
use biome_sup_client as sup_client;
use biome_sup_protocol as protocol;

#[cfg(feature = "v2")]
pub mod cli;

#[cfg(feature = "v4")]
mod cli_v4;

#[cfg(feature = "v4")]
pub use cli_v4::cli_driver;

pub mod command;
pub mod error;
mod exec;
pub mod license;
pub mod scaffolding;

pub const PRODUCT: &str = "bio";
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
pub const ORIGIN_ENVVAR: &str = "HAB_ORIGIN";
pub const BLDR_URL_ENVVAR: &str = "HAB_BLDR_URL";
pub const AFTER_HELP: &str =
    "\nALIASES:\n    apply      Alias for: 'config apply'\n    install    Alias for: 'pkg \
     install'\n    run        Alias for: 'sup run'\n    setup      Alias for: 'cli setup'\n    \
     start      Alias for: 'svc start'\n    stop       Alias for: 'svc stop'\n    term       \
     Alias for: 'sup term'\n";

pub use crate::hcore::AUTH_TOKEN_ENVVAR;

// TODO:agadgil: When Clap v2 support is gone, this should become `pub(crate)`
pub mod key_type;
