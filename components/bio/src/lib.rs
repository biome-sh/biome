#![recursion_limit = "128"]

use biome_api_client as api_client;
use biome_common as common;
use biome_core as hcore;
use biome_sup_client as sup_client;
use biome_sup_protocol as protocol;

pub mod cli;
pub mod command;
pub mod error;
mod exec;
pub mod license;
pub mod scaffolding;

pub const PRODUCT: &str = "bio";
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
pub const ORIGIN_ENVVAR: &str = "HAB_ORIGIN";
pub const BLDR_URL_ENVVAR: &str = "HAB_BLDR_URL";

pub use crate::hcore::AUTH_TOKEN_ENVVAR;
