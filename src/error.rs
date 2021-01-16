use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    /// Some unspecified error.
    Any(Box<dyn StdError + Send + Sync + 'static>),
    /// http request to Roku device did not respond with OK 200 status.
    Non200Status { url: String, code: u16 },
}
