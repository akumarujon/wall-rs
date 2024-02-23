use std::path::Path;
use wall_rs::config::Config;

static CONFIG: &str = "./config.toml";

fn main() {
    let conf = Config::new("/Users/sakhib/Developer/github/wall-rs".to_string(), 32);

    conf.write(CONFIG.to_string()).unwrap();

    let another_one = Config::from_file(&Path::new(CONFIG));

    println!("Read file: {:?}", another_one);
}
