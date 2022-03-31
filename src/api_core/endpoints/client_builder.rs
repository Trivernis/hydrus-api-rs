use crate::error::{Error, Result};
use crate::Client;
use std::time::Duration;

pub struct ClientBuilder {
    reqwest_builder: reqwest::ClientBuilder,
    base_url: String,
    access_key: Option<String>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            reqwest_builder: Default::default(),
            base_url: "127.0.0.1:45869".to_string(),
            access_key: None,
        }
    }
}

impl ClientBuilder {
    /// Set the base url with port for the client api
    /// The default value is `127.0.0.1:45869`
    pub fn url<S: ToString>(mut self, url: S) -> Self {
        self.base_url = url.to_string();

        self
    }

    /// Sets the access key for the client.
    /// The key is required
    pub fn access_key<S: ToString>(mut self, key: S) -> Self {
        self.access_key = Some(key.to_string());

        self
    }

    /// Sets the default timeout for requests to the API
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.reqwest_builder = self.reqwest_builder.timeout(timeout);

        self
    }

    /// Builds the client
    pub fn build(self) -> Result<Client> {
        let access_key = self
            .access_key
            .ok_or_else(|| Error::BuildError(String::from("missing access key")))?;
        Ok(Client {
            inner: self.reqwest_builder.build()?,
            base_url: self.base_url,
            access_key,
        })
    }
}
