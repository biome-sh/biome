use crate::{cli_v4::utils::CacheKeyPath, command::ring::key::import::start, error::Result as BioResult};
use biome_common::ui::UI;
use biome_core::crypto::{init, keys::KeyCache};
use clap::Parser;
use clap_v4 as clap;
use std::{
    io::{self, Read},
    path::PathBuf,
};

#[derive(Debug, Clone, Parser)]
#[command(help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) struct RingKeyImportOpts {
    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

impl RingKeyImportOpts {
    pub(crate) async fn do_import(&self, ui: &mut UI) -> BioResult<()> {
        let mut content = String::new();
        io::stdin().read_to_string(&mut content)?;
        let key_path: PathBuf = (&self.cache_key_path).into();
        let key_cache = KeyCache::new(key_path);
        key_cache.setup()?;
        init()?;
        start(ui, content.trim(), &key_cache)
    }
}
