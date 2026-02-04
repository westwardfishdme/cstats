use clap::{Args, Parser, Subcommand};
use clap_stdin::FileOrStdin;

#[derive(Clone, Debug, Parser)]
#[clap(version)]

pub struct BaseArgs {
    #[command(subcommand)]
    pub subcommands: StatsCommands,
}

#[derive(Clone, Debug, Subcommand)]
pub enum StatsCommands {
    /// Perform Basic Statistic Calculations
    Stats(StatArgs),
}

#[derive(Clone, Debug, Subcommand)]
pub enum StatSubcommands {
    /// Prints of all relative statistical information in a dataset
    /// file or stdin containing the dataset
    All(ArgFile),
    /// Print the sum only
    Sum(ArgFile),
}

#[derive(Clone, Debug, Args)]
pub struct StatArgs {
    #[arg(long, short)]
    #[clap(default_value = "txt")]
    /// Read the output from a specified format.
    /// Currently supported: `csv`, `txt`
    pub in_format: String,

    #[command(subcommand)]
    pub subcommands: StatSubcommands,
}

#[derive(Clone, Debug, Args)]
pub struct ArgFile {
    /// Dataset to parse
    pub dataset: FileOrStdin,
}
