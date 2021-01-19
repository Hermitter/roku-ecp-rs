use super::{from_str, Deserialize, Device, Error};

impl Device {
    /// Returns an icon corresponding to the application identified by appID. The binary
    /// data will contain an identifying MIME-type header.
    pub async fn icon(&self, app_id: u32) -> Result<Vec<u8>, Error> {
        Ok(self
            .http
            .get(format!("{}/query/icon/{}", self.url, app_id))
            .recv_bytes()
            .await?)
    }

    /// Returns information on the currently opened app.
    pub async fn active_app(&self) -> Result<ActiveApp, Error> {
        let response = self
            .http
            .get(format!("{}/query/active-app", self.url))
            .recv_string()
            .await?;

        Ok(from_str(&response)?)
    }

    /// Returns information on all installed apps.
    pub async fn apps(&self) -> Result<Apps, Error> {
        let response = self
            .http
            .get(format!("{}/query/apps", self.url))
            .recv_string()
            .await?;

        Ok(from_str(&response)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct Apps {
    #[serde(rename = "app")]
    pub apps: Vec<App>,
}

#[derive(Debug, Deserialize)]
pub struct App {
    pub id: String,
    #[serde(rename = "$value")]
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct ActiveApp {
    pub app: App,
    pub screensaver: Option<Screensaver>,
}

#[derive(Debug, Deserialize)]
pub struct Screensaver {
    pub id: String,
    #[serde(rename = "$value")]
    pub name: String,
    #[serde(rename = "type")]
    pub screensaver_type: String,
    pub version: String,
}
