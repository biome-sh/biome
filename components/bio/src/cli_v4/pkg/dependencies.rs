// Implemenatation of `bio pkg dependencies`

use clap_v4 as clap;

use clap::{ArgAction,
           Parser};

use biome_common::cli::clap_validators::BioPkgIdentValueParser;

use biome_core::{fs::FS_ROOT_PATH,
                   package::PackageIdent};

use crate::{command::pkg::{dependencies,
                           DependencyRelation,
                           Scope},
            error::Result as BioResult};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgDependenciesOptions {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT", value_parser = BioPkgIdentValueParser::simple())]
    pkg_ident: PackageIdent,

    /// Show transitive dependencies
    #[arg(name = "TRANSITIVE", short = 't', long = "transitive", action= ArgAction::SetTrue)]
    transitive: bool,

    /// Show packages which are dependant on this one
    #[arg(name = "REVERSE", short = 'r', long = "reverse", action = ArgAction::SetTrue)]
    reverse: bool,
}

impl PkgDependenciesOptions {
    pub(super) fn do_dependencies(&self) -> BioResult<()> {
        let scope = if self.transitive {
            Scope::PackageAndDependencies
        } else {
            Scope::Package
        };

        let relation = if self.reverse {
            DependencyRelation::Supports
        } else {
            DependencyRelation::Requires
        };

        dependencies::start(&self.pkg_ident, scope, relation, &*FS_ROOT_PATH).map_err(Into::into)
    }
}
