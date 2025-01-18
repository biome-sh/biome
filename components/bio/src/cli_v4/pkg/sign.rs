// Implementation of `bio pkg sign` command

use clap_v4 as clap;

use std::path::PathBuf;

use clap::Parser;

use biome_core::{crypto,
                   crypto::keys::KeyCache,
                   origin::Origin};

use biome_common::{cli::clap_validators::{FileExistsValueParser,
                                            BioOriginValueParser},
                     cli_config::CliConfig,
                     ui::UI};

use crate::{cli_v4::utils::CacheKeyPath,
            command::pkg::sign,
            error::{Error as BioError,
                    Result as BioResult}};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgSignOptions {
    /// Origin key used to create signature
    #[arg(name = "ORIGIN", long = "origin", env=crate::ORIGIN_ENVVAR, value_parser = BioOriginValueParser)]
    origin: Option<Origin>,

    // TODO: Move to semantic PathBuf after CLAP-v2 support is removed kept due to Clap V2 quirk
    /// A path to a source archive file (ex: /home/acme-redis-3.0.7-21120102031201.tar.xz)
    #[arg(name = "SOURCE", value_parser = FileExistsValueParser)]
    source: String,

    /// The destination path to the signed Biome Artifact (ex:
    /// /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
    #[arg(name = "DEST")]
    dest: PathBuf,

    #[command(flatten)]
    cache_key_path: CacheKeyPath,
}

impl PkgSignOptions {
    pub(crate) fn do_sign(&self, ui: &mut UI) -> BioResult<()> {
        let origin = match &self.origin {
            Some(origin) => origin.clone(),
            None => {
                CliConfig::load()?.origin.ok_or_else(|| {
                                              BioError::CryptoCLI("No origin specified".to_string())
                                          })?
            }
        };

        crypto::init()?;
        let key_cache = KeyCache::new::<PathBuf>((&self.cache_key_path).into());
        let key = key_cache.latest_secret_origin_signing_key(&origin)?;
        sign::start(ui,
                    &key,
                    &Into::<PathBuf>::into(self.source.clone()),
                    &self.dest)
    }
}
