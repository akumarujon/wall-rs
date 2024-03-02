use octocrab::Error as OctoError;
use std::fmt::{Debug, Formatter};

pub enum Error {
    FileOpenError(std::io::Error),
    SerializeError(String),
    DeserializeError(String),
    ReadConfigError(std::io::Error),
    WriteConfigError(std::io::Error),
    NoCorrespondingPathError,
    ReadDirError(std::io::Error),
    PathBufParseError(String),
    NotListableDirectory(String),
    ReleaseFetchError(OctoError),
    NoVersionFound,
    NoInternetConnection,
    CantCreateDownloadedFile(String),
    CantCreateCursorBytes,
    CantCopyBytes,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileOpenError(io) => write!(f, "Failed on trying to open file: {}", io),
            Error::SerializeError(target) => write!(f, "Failed while serializing data: {}", target),
            Error::DeserializeError(target) => write!(f, "Failed while deserializing data: {}", target),
            Error::ReadConfigError(io) => write!(f, "Failed while trying to read config: {}", io),
            Error::WriteConfigError(io) => write!(f, "Failed while trying to write data to a file: {}", io),
            Error::NoCorrespondingPathError => write!(
                f,
                "Failed while trying to find config file, probably directories namely doesn't exist."
            ),
            Error::ReadDirError(io) => write!(f, "Failed while trying read contents in a directory: {}", io),
            Error::PathBufParseError(target) => write!(f, "Can't parse the path: {}", target),
            Error::NotListableDirectory(target) => write!(f, "The following {} directory can't be listed", target),
            Error::ReleaseFetchError(trace) => write!(f, "Failed to fetch releases: {}", trace),
            Error::NoVersionFound => write!(f, "Couldn't get latest version from list"),
            Error::NoInternetConnection => write!(f, "Seems like you don't have access to Internet, right?)"),
            Error::CantCreateDownloadedFile(target) => write!(f, "Couldn't create the file {}", target),
            Error::CantCreateCursorBytes => write!(f, "Can't create cursor bytes for response"),
            Error::CantCopyBytes => write!(f, "Couldn't copy downloaded bytes to file")
        }
    }
}
