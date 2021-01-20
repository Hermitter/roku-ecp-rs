use std::{error::Error as StdError, fmt};

/// Crate errors.
#[derive(Debug)]
pub enum Error {
    /// Some unspecified error.
    Any(Box<dyn StdError + Send + Sync + 'static>),
    /// HTTP request to Roku device did not respond with OK 200 status.
    Non200Status { status: u16 },
    /// Cannot derive a struct from the given string.
    UnableToDerive { description: String },
    /// Tried to parse an invalid URL.
    InvalidUrl(String),
}

impl<'a> fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Non200Status { status } => {
                write!(f, "Request returned status code: {}", status)
            }
            Error::UnableToDerive { description } => {
                write!(f, "Unable to derive struct from:\n {}", description)
            }
            Error::InvalidUrl(bad_url) => {
                write!(f, "Tried to parse an invalid URL: {}", bad_url)
            }
            _ => write!(f, "TODO"),
        }
    }
}

use serde_xml_rs;
impl From<serde_xml_rs::Error> for Error {
    fn from(err: serde_xml_rs::Error) -> Self {
        Error::UnableToDerive {
            description: format!("{:?}", err),
        }
    }
}

use surf;
impl From<surf::Error> for Error {
    fn from(res: surf::Error) -> Self {
        Error::Non200Status {
            status: res.status().into(),
        }
    }
}

use url;
impl From<url::ParseError> for Error {
    fn from(bad_url: url::ParseError) -> Self {
        Error::InvalidUrl(bad_url.to_string())
    }
}
