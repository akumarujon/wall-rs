use crate::config::Config;

pub struct Wall {
    config: Config
}

impl Wall {
    pub fn new(config: Config) -> Wall {
        Wall {
            config
        }
    }

}