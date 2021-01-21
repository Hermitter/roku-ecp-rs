#![doc(
    html_logo_url = "https://raw.githubusercontent.com/hermitter/roku-ecp-rs/assets/roku-icon.svg?sanitize=true"
)]

pub use error::Error;
use surf::Client;
mod api;
mod error;
pub use api::keys::Key;
pub use api::search::{SearchRequest, SearchType};
use url::Url;

/// HTTP port for communicating with the ECP RESTful service.
const ECP_PORT: &str = "8060";

/// A Roku device to communicate with via the [External Control Protocol](https://developer.roku.com/docs/developer-program/debugging/external-control-api.md).
#[derive(Debug)]
pub struct Device {
    /// Base URL of the Roku device's IP and port.
    pub url: url::Url,
    /// Http client for communicating with Roku device.
    http: Client,
}

impl Device {
    /// Constructs a client to communicate with a Roku device. This assumes the device is on the local network.
    pub fn new<T>(ip: T) -> Result<Device, Error>
    where
        T: std::net::ToSocketAddrs + std::fmt::Display,
    {
        Ok(Device {
            url: Url::parse(&format!("http://{}:{}", ip, ECP_PORT))?,
            http: Client::new(),
        })
    }

    /// Constructs a client to communicate with a Roku at the specified URL .
    pub fn from_url(url: &str) -> Result<Device, Error> {
        Ok(Device {
            url: Url::parse(url)?,
            http: Client::new(),
        })
    }

    /// Ping the Roku device to test the connection.
    pub async fn ping(&self) -> Result<(), Error> {
        self.http.get(&self.url).send().await?;
        Ok(())
    }
}
