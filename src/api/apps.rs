use crate::Device;
use crate::Error;
use std::fmt::Display;
use url::form_urlencoded;

impl Device {
    /// Launches the channel identified by appID and passes it any available parameters.
    ///
    /// Note that this command will not launch uninstalled channels.
    pub async fn launch<'a, T>(
        &self,
        app_id: T,
        parameters: Option<&[(&'a str, &'a str)]>,
    ) -> Result<(), Error>
    where
        T: Display,
    {
        let mut url = self.url.join(&format!("launch/{}", app_id))?;

        let query = parameters
            .unwrap_or(&[])
            .into_iter()
            .fold(
                form_urlencoded::Serializer::new(String::new()),
                |mut query, q| {
                    query.append_pair(q.0, q.1);
                    query
                },
            )
            .finish();

        url.set_query(Some(&query));

        self.http.post(url).await?;
        Ok(())
    }
}
