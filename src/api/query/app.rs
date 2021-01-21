use super::{from_str, Deserialize, Device, Error};

impl Device {
    /// Returns an icon corresponding to the Roku application identified by appID.
    pub async fn icon(&self, app_id: u32) -> Result<Icon, Error> {
        let mut request = self
            .http
            .get(self.url.join(&format!("query/icon/{}", app_id))?)
            .send()
            .await?;

        let bytes = request.body_bytes().await?;

        let mime_type: String = match request.header("content-type") {
            Some(m) => m[0].to_string(),
            // [octet-steam](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types#applicationoctet-stream)
            // is used to describe an unknown binary file.
            None => String::from("application/octet-stream"),
        };

        Ok(Icon { bytes, mime_type })
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

    /// Returns a list of all currently installed apps.
    pub async fn apps(&self) -> Result<Vec<App>, Error> {
        let response = self
            .http
            .get(format!("{}/query/apps", self.url))
            .recv_string()
            .await?;

        Ok(from_str::<Apps>(&response)?.apps)
    }
}

/// An icon that corresponds to a Roku application.
#[derive(Debug)]
pub struct Icon {
    /// The [MIME-type](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
    /// describing the icon's file type.
    pub mime_type: String,
    /// The byte representation of the icon.
    pub bytes: Vec<u8>,
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
