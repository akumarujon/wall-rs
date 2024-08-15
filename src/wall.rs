use crate::config::Config;
use crate::error::Error;
use crate::showroom;
use crate::source::Source;
use rand::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use path_absolutize::*;

pub struct Wall {
    pub config: Option<Config>,
}

impl Wall {
    pub fn new(config: Option<Config>) -> Wall {
        Wall { config }
    }

    pub fn absolute(&self, path: PathBuf) -> PathBuf {
        match path.absolutize() {
            Ok(p) => p.to_path_buf(),
            Err(_) => {
                eprintln!("Can't not absolutize the path.");
                exit(1);
            }
        }
    }

    pub fn set(&self, path: PathBuf) {
        let binding = self.absolute(path.clone());

        let path = match binding.to_str() {
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

        location = self.absolute(location.clone());

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

    pub async fn install(&mut self, url: Option<String>) -> Result<(), Error> {
        let mut target = String::new();
        let source = Source::new(None);

        // Download link:
        // https://github.com/akumarujon/wall-rs-mirror/releases/tag/v0.0.3
        // https://github.com/akumarujon/wall-rs-mirror/releases/download/v0.0.3/assets.zip

        match url {
            Some(l) => target.push_str(&l),
            None => {
                let mut versions = source.get_latest_version().await.unwrap();
                versions = format!(
                    "https://github.com/akumarujon/wall-rs-mirror/releases/download/{}/assets.zip",
                    versions
                );

                self.config.as_mut().unwrap().set_version(versions.clone());
                target.push_str(&versions)
            }
        };

        const FILENAME: &str = "assets.zip";

        source
            .download_file(target, FILENAME.to_string())
            .await
            .unwrap();

        source.extract_file(FILENAME).unwrap();

        Ok(())
    }

    pub fn auto(&self) {
        dbg!("Trying to setup wallpaper automatically!");
    }

    pub fn exit(&self) {
        self.config.as_ref().unwrap().write("").unwrap()
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
