// Implementation of `bio pkg verify` command

use clap_v4 as clap;

use std::path::PathBuf;

use clap::Parser;

use biome_core::{crypto,
                   crypto::keys::KeyCache};

use biome_common::{cli::clap_validators::FileExistsValueParser,
                     ui::UI};

use crate::{cli_v4::utils::CacheKeyPath,
            command::pkg::verify,
            error::Result as BioResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgVerifyOptions {
    // TODO: Move to semantic PathBuf once Clap-v2 is removed
    /// A path to a Biome Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[arg(name = "SOURCE", value_parser = FileExistsValueParser)]
    source: String,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

impl PkgVerifyOptions {
    pub(super) fn do_verify(&self, ui: &mut UI) -> BioResult<()> {
        crypto::init()?;
        let key_cache = KeyCache::new::<PathBuf>((&self.cache_key_path).into());

        verify::start(ui, &Into::<PathBuf>::into(self.source.clone()), &key_cache)
    }
}
