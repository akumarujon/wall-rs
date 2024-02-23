use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;

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
