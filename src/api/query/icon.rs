use super::*;

impl Device {
    /// Returns an icon corresponding to the application identified by appID. The binary data will contain
    /// and identifying MIME-type header.
    pub async fn app_icon(&self, app_id: u32) -> Result<Vec<u8>, Error> {
        let response = self
            .http
            .get(format!("{}/query/icon/{}", self.url, app_id))
            .recv_bytes()
            .await?;

        Ok(response)
    }
}
