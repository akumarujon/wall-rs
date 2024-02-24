pub mod config;
pub mod error;
pub mod wall;

use std::path::PathBuf;
use clap::{Parser, Subcommand};

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
}
