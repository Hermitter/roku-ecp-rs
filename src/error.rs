use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    /// Some unspecified error.
    Any(Box<dyn StdError + Send + Sync + 'static>),
    /// http request to Roku device did not respond with OK 200 status.
    Non200Status { status: u16 },
    /// Cannot derive a struct from the given string.
    UnableToDerive { description: String },
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
