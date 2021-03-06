//! # Roku ECP
//!
//! This crate provides a wrapper over the [Roku External Control
//! Protocol](https://developer.roku.com/docs/developer-program/debugging/external-control-api.md)
//! which enables a Roku device to be controlled over a local area network
//! through a RESTful API.
//!
//! # Example Usage
//! The crate is fairly straightforward since all relevant methods revolve
//! around the [Device](crate::Device) struct.
//!
//! For a full code representation, view the [examples folder](https://github.com/Hermitter/roku-ecp-rs/tree/main/examples).
//! ```ignore
//! use roku_ecp::{Device, Key, SearchRequest, SearchType};
//! let roku = Device::new("192.168.1.138").unwrap(); // remember to change the IP.
//!
//! // Print information on the Roku device
//! println!("{:?}", roku.device_info().await.unwrap());
//!
//! // Press the play button
//! roku.key_press(Key::Play).await.unwrap();
//!
//! // Search for something
//! let search = SearchRequest::new("The Mandalorian")
//!     .search_type(SearchType::TvShow)
//!     .season(2)
//!
//! roku.search(search).await.unwrap();
//! ```
//!
//! # Features
//! Currently Implemented:
//! - Queries for device, media player, and app information.
//! - Key press/down/up events for various [keys](crate::Key).
//! - Launching/Installing Applications.
//! - Searching through Roku's Search UI.
//!
//! Possible Features for the Future:
//! - Scanning the local network for Roku devices.
//! - Inputs for  accelerometer, orientation, gyroscope, magnetometer, touch,
//!   and multi-touch.
//! - Roku TV commands for queries and inputs.
//!
//! # Dependencies
//!
//! **Operating Systems**
//!
//! Windows and macOS: None
//!
//! Linux: OpenSSL + headers > v1.0.1
//! - Debian: `sudo apt install libssl-dev`
//! - Fedora: `sudo dnf install openssl-devel`
//!
//! **Async Runtime**
//!
//! This crate requires an asynchronous runtime such as
//! [tokio](https://github.com/tokio-rs/tokio) or
//! [async-std](https://github.com/async-rs/async-std).
//!
//!

pub use error::Error;
use surf::Client;
mod api;
mod error;
pub use api::keys::Key;
pub use api::search::{SearchRequest, SearchType};
use url::Url;

/// Default port for communicating with the ECP RESTful service.
pub const ECP_PORT: u16 = 8060;

/// An HTTP client that communicates with a Roku device.
#[derive(Debug)]
pub struct Device {
    /// Base URL of the Roku device's IP and port.
    url: url::Url,
    /// Http client for communicating with Roku device.
    http: Client,
}

impl Device {
    /// Create an HTTP client to communicate with a Roku device. This assumes
    /// the device URL is `http://ROKU_IP_HERE:8060`.
    pub fn new<T>(ip: T) -> Result<Device, Error>
    where
        T: std::net::ToSocketAddrs + std::fmt::Display,
    {
        Ok(Device {
            url: Url::parse(&format!("http://{}:{}", ip, ECP_PORT))?,
            http: Client::new(),
        })
    }

    /// Creates an HTTP client with a specified URL to communicate with a Roku
    /// device. Use this if the default URL
    /// (`http://ROKU_IP_HERE:8060`) is not usable to you.
    pub fn from_url(url: &str) -> Result<Device, Error> {
        Ok(Device {
            url: Url::parse(url)?,
            http: Client::new(),
        })
    }

    /// Pings the Roku device to check if the device is online.
    pub async fn ping(&self) -> Result<(), Error> {
        self.http.get(&self.url).send().await?;
        Ok(())
    }
}
