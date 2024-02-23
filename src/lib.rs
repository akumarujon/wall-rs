pub mod config;
pub mod error;
pub mod wall;

use clap::{Parser, Subcommand};

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "wall")]
#[command(about = "Wallpaper manager for you", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Clones repos
    #[command(arg_required_else_help = true)]
    Set {
        /// The remote to clone
        remote: String,
    },
}