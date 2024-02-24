use crate::config::Config;
use crate::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use rand::prelude::*;

pub struct Wall {
    pub config: Option<Config>,
}

impl Wall {
    pub fn new(config: Option<Config>) -> Wall {
        Wall { config }
    }

    pub fn set(&self, path: PathBuf) {
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

    fn get_pics(dir: &PathBuf) -> Result<Vec<PathBuf>, Error> {
        let mut result: Vec<PathBuf> = Vec::new();

        if dir.is_dir() {
            let read_dir = match fs::read_dir(dir) {
                Ok(e) => e,
                Err(_) => return Err(Error::ReadDirError),
            };

            for entry in read_dir {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_dir() {
                    Self::get_pics(&path).unwrap();
                } else {
                    let check = path.to_str().unwrap();
                    if check.ends_with(".png") || check.ends_with(".jpg") {
                        result.push(path)
                    }
                }
            }
        }

        Ok(result)
    }

    pub fn random(&self, path: Option<PathBuf>) {
        let mut rng = rand::thread_rng();
        let mut location = PathBuf::new();

        if let Some(c) = &self.config {
            location = PathBuf::from_str(&c.path).unwrap()
        }

        if path.is_some() {
            location = path.unwrap()
        }

        let mut pics = Self::get_pics(&location).unwrap();

        pics.shuffle(&mut rng);

        self.set(pics.first().unwrap().to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::wall::Wall;

    #[test]
    fn test_random() {
        let config = match Config::from_local_conf() {
            Ok(c) => Some(c),
            Err(_) => None,
        };
        let wall = Wall::new(config);

        wall.random(None);
    }
}
