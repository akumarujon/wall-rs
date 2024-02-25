use crate::error::Error;
use crate::matcher;
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
    // pub mode: wallpaper::Mode
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
            true => Ok(match PathBuf::from_str(&result) {
                Ok(p) => p,
                Err(err) => return Err(Error::PathBufParseError(err.to_string())),
            }),
            false => return Err(Error::NoCorrespondingPathError),
        };
    }

    pub fn from_local_conf() -> Result<Config, Error> {
        let config_path = match dirs::config_dir() {
            Some(p) => p,
            None => return Err(Error::NoCorrespondingPathError),
        };

        let config_dir = matcher!(Config::walk(config_path.to_str().unwrap(), "wall"));

        let config = matcher!(Config::walk(config_dir.to_str().unwrap(), "config.toml"));

        let read = matcher!(Config::from_file(config));

        Ok(read)
    }

    pub fn from_file<T>(path: T) -> Result<Config, Error>
    where
        T: AsRef<Path>,
    {
        let file = File::open(&path);

        let mut file = match file {
            Ok(file) => file,
            Err(err) => {
                return Err(Error::FileOpenError(err));
            }
        };

        let mut content = String::new();

        match file.read_to_string(&mut content) {
            Ok(_) => dbg!("Successfully read the data"),
            Err(err) => return Err(Error::ReadConfigError(err)),
        };

        let config = toml::from_str::<Config>(&content);

        let result = match config {
            Ok(d) => d,
            Err(err) => return Err(Error::SerializeError(err.to_string())),
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
            Err(err) => return Err(Error::WriteConfigError(err)),
        };

        let deser = match toml::to_string_pretty(self) {
            Ok(s) => s,
            Err(err) => return Err(Error::DeserializeError(err.to_string())),
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
