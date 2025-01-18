use clap_v4 as clap;

use clap::Parser;

use biome_common::{ui::UI,
                     FeatureFlag};

use crate::{error::Result as BioResult,
            AFTER_HELP,
            VERSION};

mod pkg;
use pkg::PkgCommand;

mod utils;
use utils::CacheKeyPath;

#[derive(Debug, Clone, Parser)]
#[command(name = "bio",
            version = VERSION,
            about = "\n\"A Biome is the natural environment for your services\" - Alan Turing",
            author = "\nThe Biome Maintainers <humans@biome.sh>",
            after_help = AFTER_HELP,
            arg_required_else_help = true,
            propagate_version = true,
            help_template = "{name} {version} {author-section} {about-section} \
                    \n{usage-heading} {usage}\n\n{all-args}\n{after-help}\n",
        )]
enum Bio {
    /// Commands relating to Biome Builder
    Bldr(BldrCommand),

    /// Commands relating to Biome runtime config
    Cli(CliCommand),

    Config(ConfigCommand),

    File(FileCommand),

    License(LicenseCommand),

    Origin(OriginCommand),

    /// Commands relating to Biome packages
    #[clap(subcommand)]
    Pkg(PkgCommand),

    Plan(PlanCommand),

    Ring(RingCommand),

    Studio(StudioCommand),

    Sup(SupCommand),

    SupportBundle,

    Svc(SvcCommand),

    User(UserCommand),

    // Aliases Below
    Apply(ServiceConfigCommand),

    Install(PkgInstallCommand),

    Run(SupRunCommand),

    Setup(CacheKeyPath),

    Start(SvcStartCommand),

    Stop(SvcStopCommand),

    Term,
}

impl Bio {
    async fn do_cli_command(&self, ui: &mut UI, feature_flags: FeatureFlag) -> BioResult<()> {
        match self {
            Self::Pkg(pkg_command) => pkg_command.do_command(ui, feature_flags).await,
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug, Parser)]
pub(crate) struct BldrCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct CliCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct ConfigCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct FileCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct LicenseCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct OriginCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct PlanCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct RingCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct StudioCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SupCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SvcCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct UserCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct ServiceConfigCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct PkgInstallCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SupRunCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SvcStartCommand;

#[derive(Clone, Debug, Parser)]
pub(crate) struct SvcStopCommand;

pub async fn cli_driver(ui: &mut UI, feature_flags: FeatureFlag) -> BioResult<()> {
    let cli = Bio::parse();
    cli.do_cli_command(ui, feature_flags).await
}
