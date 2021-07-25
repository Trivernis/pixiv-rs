use std::fmt;
use std::error::Error as StdError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Pixiv(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Reqwest(e) => {e.fmt(f)}
            Error::Pixiv(s) => {write!(f, "Pixiv error: {}", s)}
        }
    }
}

impl StdError for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}