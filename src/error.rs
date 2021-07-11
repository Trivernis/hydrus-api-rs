use std::error::Error as StdError;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Hydrus(String),
    InvalidServiceType(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reqwest(e) => e.fmt(f),
            Self::Hydrus(msg) => msg.fmt(f),
            Self::InvalidServiceType(service_type) => {
                write!(f, "Invalid Service Type '{}'", service_type)
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Reqwest(e) => e.source(),
            Self::Hydrus(_) => None,
            Self::InvalidServiceType(_) => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}
