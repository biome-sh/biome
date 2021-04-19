use super::util::{CacheKeyPath,
                  ConfigOptCacheKeyPath};
use configopt::ConfigOpt;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Biome rings
pub enum Ring {
    Key(Key),
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Biome ring keys
pub enum Key {
    Export(RingKeyExport),
    Generate(RingKeyGenerate),
    Import(RingKeyImport),
}

/// Outputs the latest ring key contents to stdout
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "export", no_version)]
pub struct RingKeyExport {
    /// Ring key name
    #[structopt(name = "RING")]
    ring:           String,
    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Generates a Biome ring key
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "generate", no_version)]
pub struct RingKeyGenerate {
    /// Ring key name
    #[structopt(name = "RING")]
    ring:           String,
    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Reads a stdin stream containing ring key contents and writes the key to disk
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "import", no_version)]
pub struct RingKeyImport {
    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}
