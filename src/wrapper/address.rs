use crate::api_core::common::OptionalStringNumber;
use crate::api_core::managing_cookies_and_http_headers::CookieBuilder;
use crate::error::Result;
use crate::Client;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Address {
    client: Client,
    domain: String,
    path: String,
}

impl Address {
    pub(crate) fn from_str(client: Client, domain: &str) -> Self {
        let (domain, path) = domain.split_once("/").unwrap_or((domain, "/"));
        Self {
            client,
            domain: domain.to_string(),
            path: path.to_string(),
        }
    }

    /// Returns the path after the domain name
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Sets the path of the domain that can be used for setting cookies
    pub fn set_path<S: ToString>(&mut self, path: S) {
        self.path = path.to_string();
    }

    /// Sets cookies for the domain
    pub async fn set_cookies(&self, cookies: Vec<DomainCookie>) -> Result<()> {
        let cookies = cookies
            .into_iter()
            .map(|cookie| {
                let mut builder = CookieBuilder::default()
                    .domain(&self.domain)
                    .path(&self.path)
                    .name(cookie.name)
                    .value(cookie.value);
                if let Some(expires) = cookie.expires {
                    builder =
                        builder.expires(expires.duration_since(UNIX_EPOCH).unwrap().as_secs());
                }
                builder.build()
            })
            .collect();

        self.client.set_cookies(cookies).await
    }

    /// Returns all cookies stored for this domain
    pub async fn get_cookies(&self) -> Result<Vec<DomainCookie>> {
        let response = self.client.get_cookies(&self.domain).await?;
        let cookies = response
            .cookies
            .into_iter()
            .map(DomainCookie::from)
            .collect();

        Ok(cookies)
    }
}

#[derive(Clone, Debug)]
pub struct DomainCookie {
    pub name: String,
    pub value: String,
    pub expires: Option<SystemTime>,
}

impl DomainCookie {
    /// Creates a new cookie that will be expire after the given instant or only last for the session
    pub fn new<S1: ToString, S2: ToString>(
        name: S1,
        value: S2,
        expires: Option<SystemTime>,
    ) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            expires,
        }
    }
}

impl From<[OptionalStringNumber; 5]> for DomainCookie {
    fn from(cookie_entry: [OptionalStringNumber; 5]) -> Self {
        let name = cookie_entry[0].string().unwrap_or("");
        let value = cookie_entry[1].string().unwrap_or("");
        let expires = cookie_entry[4]
            .number()
            .map(|n| UNIX_EPOCH + Duration::from_secs(n));

        Self::new(name, value, expires)
    }
}
