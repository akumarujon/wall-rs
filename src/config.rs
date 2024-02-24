use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub path: String,
    pub interval: u32,
}

impl Config {
    pub fn new<T>(path: T, interval: u32) -> Config
    where
        T: ToString,
    {
        Config {
            path: path.to_string(),
            interval,
        }
    }

    pub fn walk<T>(path: T, to: T) -> Result<PathBuf, Error>
    where
        T: AsRef<str>,
    {
        let mut result = String::new();

        result.push_str(path.as_ref());

        if !result.ends_with('/') {
            result.push('/')
        }

        result.push_str(to.as_ref());

        return match Path::new(&result).exists() {
            true => Ok(PathBuf::from_str(&result).unwrap()),
            false => return Err(Error::NoCorrespondingPathError),
        };
    }

    pub fn from_local_conf() -> Result<Config, Error> {
        let config_path = match dirs::config_dir() {
            Some(p) => p,
            None => return Err(Error::NoCorrespondingPathError),
        };

        let config_dir = match Config::walk(config_path.to_str().unwrap(), "wall") {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        let config = match Config::walk(config_dir.to_str().unwrap(), "config.toml") {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        let read = match Config::from_file(config) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(read)
    }

    pub fn from_file<T>(path: T) -> Result<Config, Error>
    where
        T: AsRef<Path>,
    {
        let file = File::open(&path);

        let mut file = match file {
            Ok(file) => file,
            Err(_err) => {
                // let msg = format!("Error opening file {}: {}", path, err);
                return Err(Error::FileOpenError);
            }
        };

        let mut content = String::new();

        file.read_to_string(&mut content)
            .expect("Couldn't read file to string");

        let config = toml::from_str::<Config>(&content);

        let result = match config {
            Ok(d) => d,
            Err(_) => return Err(Error::SerializeError),
        };

        Ok(result)
    }

    pub fn write<T>(&self, path: T) -> Result<(), Error>
    where
        T: ToString + Clone,
    {
        let path = path.to_string();
        let path = Path::new(&path);

        if path.exists() {
            std::fs::remove_file(path).expect("TODO: panic message");
        }

        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path);

        let mut file = match file {
            Ok(f) => f,
            Err(_) => return Err(Error::WriteConfigError),
        };

        let deser = match toml::to_string_pretty(self) {
            Ok(s) => s,
            Err(_) => return Err(Error::DeserializeError),
        };

        file.write_all(deser.as_bytes())
            .expect("Failed writing to a file");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn some() {
        let config = Config::from_local_conf();

        println!("{:?}", config);
    }
}
