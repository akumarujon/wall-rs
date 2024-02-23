use std::fmt::{Debug, Formatter};

pub enum Error {
    FileOpenError,
    ReadError,
    SerializeError,
    DeserializeError,
    OpenConfigError,
    WriteConfigError,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileOpenError => write!(f, "Failed on trying to open file"),
            Error::ReadError => write!(f, "Failed while reading content to string"),
            Error::SerializeError => write!(f, "Failed while serializing data"),
            Error::DeserializeError => write!(f, "Failed while deserializing data"),
            Error::OpenConfigError => write!(f, "Failed while trying to read config"),
            Error::WriteConfigError => write!(f, "Failed while trying to write data to a file"),
        }
    }
}