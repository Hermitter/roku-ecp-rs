use crate::Device;
use crate::Error;
use std::fmt::Display;
use url::form_urlencoded;

impl Device {
    /// Launches a channel and passes it
    /// the parameters given . This call will only launch installed channels.
    pub async fn launch<'a, T>(
        &self,
        app_id: T,
        parameters: Option<&[(&'a str, &'a str)]>,
    ) -> Result<(), Error>
    where
        T: Display,
    {
        self.launch_action(app_id, parameters, false).await
    }

    /// Installs and launches a channel along with passing the given parameters.
    pub async fn install<'a, T>(
        &self,
        app_id: T,
        parameters: Option<&[(&'a str, &'a str)]>,
    ) -> Result<(), Error>
    where
        T: Display,
    {
        self.launch_action(app_id, parameters, true).await
    }

    /// Sends an ECP request to call to call either `/install` or `/launch` for
    /// a specific channel.
    async fn launch_action<'a, T>(
        &self,
        app_id: T,
        parameters: Option<&[(&'a str, &'a str)]>,
        install_app: bool,
    ) -> Result<(), Error>
    where
        T: Display,
    {
        let path = match install_app {
            true => "install",
            false => "launch",
        };

        let mut url = self.url.join(&format!("{}/{}", path, app_id))?;

        // create the query string to pass into the URL.
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
