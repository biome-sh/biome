//! Biome helps you build, manage, and run applications - on bare metal, in the cloud, and in
//! containers. You can [read more about it on the website](https://www.habitat.sh/).
//!
//! Biome contains two main components:
//!
//! * `bio-plan-build`, takes a plan ('plan.sh'), a description of how to build a piece of software,
//!   written in [bash](http://tldp.org/HOWTO/Bash-Prog-Intro-HOWTO.html), which produces an atomic
//!   package.
//! * `bio-sup`, a run-time executable that knows how to download, install, serve, and manage
//!   services defined in packages.
//!
//! # bio-plan-build
//!
//! The documentation for bio-plan-build is generated automatically from the script itself, [and
//! can be found here](bio-plan-build/bio-plan-build.html). You can find it in the source tree at
//! `components/plan-build`.
//!
//! # The Supervisor
//!
//! The Supervisor is primarily utilized through the `bio-sup` command; it can also be used from
//! within Rust as a library. This documentation covers both uses; it explains how things are used
//! from the command line in close proximity to the documentation of the library itself. A few
//! useful starting points:
//!
//! * [The Biome Command Line Reference](command)
//! * [The Biome Supervisor Sidecar; http interface to promises](sidecar)

#[cfg(test)]
#[macro_use]
pub mod cli_test_helpers;
pub mod census;
pub mod cli;
pub mod command;
pub mod ctl_gateway;
pub mod error;
pub mod event;
pub mod http_gateway;
pub mod lock_file;
pub mod logger; // must be pub if used in the `bio-sup` binary
pub mod manager;
mod sys;
#[cfg(test)]
pub mod test_helpers;
pub mod util;

use std::env;

pub const PRODUCT: &str = "bio-sup";
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
