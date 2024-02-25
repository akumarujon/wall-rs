pub mod config;
pub mod error;
pub mod macros;
pub mod wall;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Wallpaper manager for you
#[derive(Debug, Parser)]
#[command(name = "wall")]
#[command(about = "Wallpaper manager for you", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Set a wallpaper to your desktop
    #[command(arg_required_else_help = true)]
    Set {
        /// Path to the wallpaper
        path: PathBuf,
    },

    /// Set random picture from assets
    Random {
        /// Path to directory of assets
        path: Option<PathBuf>,
    },

    /// Generate configurations and fetch assets
    Install {
        /// Url to fetch from assets
        url: Option<String>,
    },
}
