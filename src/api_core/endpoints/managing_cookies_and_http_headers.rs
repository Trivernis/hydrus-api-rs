use crate::api_core::common::OptionalStringNumber;
use crate::api_core::endpoints::Endpoint;

#[derive(Clone, Debug, Deserialize)]
pub struct GetCookiesResponse {
    pub cookies: Vec<[OptionalStringNumber; 5]>,
}

pub struct GetCookies;

impl Endpoint for GetCookies {
    type Request = ();
    type Response = GetCookiesResponse;

    fn path() -> String {
        String::from("manage_cookies/get_cookies")
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SetCookiesRequest {
    pub cookies: Vec<[OptionalStringNumber; 5]>,
}

pub struct SetCookies;

impl Endpoint for SetCookies {
    type Request = SetCookiesRequest;
    type Response = ();

    fn path() -> String {
        String::from("manage_cookies/set_cookies")
    }
}

pub struct CookieBuilder {
    name: OptionalStringNumber,
    value: OptionalStringNumber,
    domain: OptionalStringNumber,
    path: OptionalStringNumber,
    expires: OptionalStringNumber,
}

impl Default for CookieBuilder {
    fn default() -> Self {
        Self {
            name: String::new().into(),
            value: String::new().into(),
            domain: String::new().into(),
            path: String::new().into(),
            expires: OptionalStringNumber::None,
        }
    }
}

impl CookieBuilder {
    pub fn name<S: ToString>(mut self, name: S) -> Self {
        self.name = name.to_string().into();
        self
    }

    pub fn value<S: ToString>(mut self, value: S) -> Self {
        self.value = value.to_string().into();
        self
    }

    pub fn domain<S: ToString>(mut self, domain: S) -> Self {
        self.domain = domain.to_string().into();
        self
    }

    pub fn path<S: ToString>(mut self, path: S) -> Self {
        self.path = path.to_string().into();
        self
    }

    pub fn expires(mut self, expires: u64) -> Self {
        self.expires = expires.into();
        self
    }

    pub fn build(self) -> [OptionalStringNumber; 5] {
        [self.name, self.value, self.domain, self.path, self.expires]
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SetUserAgentRequest {
    #[serde(rename = "user-agent")]
    pub user_agent: String,
}

pub struct SetUserAgent;

impl Endpoint for SetUserAgent {
    type Request = SetUserAgentRequest;
    type Response = ();

    fn path() -> String {
        String::from("manage_headers/set_user_agent")
    }
}
