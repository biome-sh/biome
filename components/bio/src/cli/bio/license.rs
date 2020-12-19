use configopt::ConfigOpt;
use structopt::StructOpt;

/// Commands relating to Biome license agreements
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "license", no_version)]
pub enum License {
    /// Accept the Biome Binary Distribution Agreement without prompting
    Accept,
}
