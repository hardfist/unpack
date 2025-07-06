
use std::path::PathBuf;

use clap::{Args, Parser, ValueEnum};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub enum Arguments {
    Build(BuildArguments)
}
#[derive(Debug, Args, Clone)]
pub struct CommonArguments {
    /// The entrypoints of the project. Resolved relative to the project's
    /// directory (`--dir`).
    #[clap(value_parser)]
    pub entries: Option<Vec<String>>,

    /// The directory of the application.
    /// If no directory is provided, the current directory will be used.
    #[clap(short, long, value_parser)]
    pub dir: Option<PathBuf>,
}
#[derive(Debug, Args)]
#[clap(author, version, about, long_about = None)]
pub struct BuildArguments {
    #[clap(flatten)]
    pub common: CommonArguments,
}
