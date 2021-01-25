use crate::Device;
use crate::Error;
use url::form_urlencoded;

impl Device {
    /// Launches a channel and passes the parameters given. This call will only launch installed channels.
    pub async fn launch<'a>(
        &self,
        app_id: &str,
        parameters: Option<&[(&'a str, &'a str)]>,
    ) -> Result<(), Error> {
        self.app_action(app_id, parameters, false).await
    }

    /// Installs and launches a channel along with passing any given parameters.
    pub async fn install<'a>(
        &self,
        app_id: &str,
        parameters: Option<&[(&'a str, &'a str)]>,
    ) -> Result<(), Error> {
        self.app_action(app_id, parameters, true).await
    }

    /// Sends an ECP request to call to call either `/install` or `/launch` for
    /// a specific channel.
    async fn app_action<'a>(
        &self,
        app_id: &str,
        parameters: Option<&[(&'a str, &'a str)]>,
        install_app: bool,
    ) -> Result<(), Error> {
        let path = match install_app {
            true => "install",
            false => "launch",
        };

        let mut url = self.url.join(&format!("{}/{}", path, app_id))?;

        // create the query string to pass into the URL.
        // NOTE: cannot confirm query strings work, despite the fact it's
        // documented.
        if let Some(query) = parameters {
            let query = query
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
        }

        self.http.post(url).await?;
        Ok(())
    }
}
