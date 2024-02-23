use std::process::exit;
use clap::Parser;
use wall_rs::{Cli, Commands};

// use std::path::Path;
// use wall_rs::config::Config;
// static CONFIG: &str = "./config.toml";
// fn main() {
//     let conf = Config::new("/Users/sakhib/Developer/github/wall-rs".to_string(), 32);
//
//     conf.write(CONFIG.to_string()).unwrap();
//
//     let another_one = Config::from_file(Path::new(CONFIG));
//
//     println!("Read file: {:?}", another_one);
// }

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Set { path } => {
            let path = match path.to_str() {
                Some(p) => p,
                None => {
                    eprintln!("Well, you passed wrong path");
                    exit(1);
                }
            };

            if wallpaper::set_from_path(path).is_ok() {
                println!("Voila!");
            } 
        }
        
        
    };
}
