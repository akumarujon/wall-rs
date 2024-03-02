use clap::Parser;
use wall_rs::config::Config;
use wall_rs::wall::Wall;
use wall_rs::{Cli, Commands};

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let config = match Config::from_local_conf() {
        Ok(c) => Some(c),
        Err(_) => None,
    };
    let mut wall = Wall::new(config);

    match args.command {
        Commands::Set { path } => wall.set(path),
        Commands::Random { path } => wall.random(path),
        Commands::Install { url } => wall.install(url).await.unwrap(),
        Commands::Auto => wall.auto(),
    };
}
