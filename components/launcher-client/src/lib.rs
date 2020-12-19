#[macro_use]
extern crate log;

mod client;
pub mod error;

pub use biome_launcher_protocol::{ERR_NO_RETRY_EXCODE,
                                    LAUNCHER_PID_ENV,
                                    OK_NO_RETRY_EXCODE};

pub use crate::{client::LauncherCli,
                error::Error};

pub fn env_pipe() -> Option<String> {
    biome_core::env::var(biome_launcher_protocol::LAUNCHER_PIPE_ENV).ok()
}
