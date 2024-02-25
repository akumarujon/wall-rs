use crate::config::Config;
use crate::error::Error;
use crate::showroom;
use rand::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

static DEFAULT_TARGET: &str = "https://github.com/akumarujon/wall-rs-mirror";

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
                Err(err) => return Err(Error::ReadDirError(err)),
            };

            for entry in read_dir {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_dir() {
                    match Self::get_pics(&path) {
                        Ok(p) => p,
                        Err(_) => {
                            return Err(Error::NotListableDirectory(String::from(
                                path.to_str().unwrap(),
                            )))
                        }
                    };
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
        let mut rng = thread_rng();
        let mut location = PathBuf::new();

        if let Some(c) = &self.config {
            location = match PathBuf::from_str(&c.path) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Can't parse the path: {}", e);
                    exit(1);
                }
            }
        }

        if path.is_some() {
            location = path.unwrap()
        }

        if location.to_str().unwrap().is_empty() {
            eprintln!(
                "Seems like we couldn't find config either you didn't pass path to assets folder..."
            );
            exit(1);
        }

        let mut pics = showroom!(Self::get_pics(&location));

        pics.shuffle(&mut rng);

        let first = match pics.first() {
            Some(p) => p,
            None => {
                eprintln!("Seems like there were no pictures!");
                exit(1)
            }
        };

        self.set(first.to_owned())
    }

    pub fn install(&self, url: Option<String>) {
        let mut target = String::new();

        match url {
            Some(l) => target.push_str(&l),
            None => target.push_str(DEFAULT_TARGET),
        };
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
