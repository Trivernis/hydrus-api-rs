use crate::error::Result;
use crate::paths::access_management::{
    ApiVersionResponse, GetServicesResponse, SessionKeyResponse, VerifyAccessKeyResponse,
};
use crate::paths::Path;
use serde::de::DeserializeOwned;
use serde::Serialize;

static ACCESS_KEY_HEADER: &str = "Hydrus-Client-API-Access-Key";

pub struct Client {
    inner: reqwest::Client,
    base_url: String,
    access_key: String,
}

impl Client {
    pub fn new<S: AsRef<str>>(url: S, access_key: S) -> Result<Self> {
        Ok(Self {
            inner: reqwest::Client::new(),
            access_key: access_key.as_ref().to_string(),
            base_url: url.as_ref().to_string(),
        })
    }

    /// Starts a get request to the path associated with the return type
    async fn get<T: DeserializeOwned + Path, Q: Serialize + ?Sized>(
        &mut self,
        query: &Q,
    ) -> Result<T> {
        let response: T = self
            .inner
            .get(format!("{}/{}", self.base_url, T::get_path()))
            .header(ACCESS_KEY_HEADER, &self.access_key)
            .query(query)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    /// Stats a post request to the path associated with the return type
    async fn post<T: DeserializeOwned + Path, B: Serialize>(&mut self, body: B) -> Result<T> {
        let response: T = self
            .inner
            .post(format!("{}/{}", self.base_url, T::get_path()))
            .json(&body)
            .header(ACCESS_KEY_HEADER, &self.access_key)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    /// Returns the current API version. It's being incremented every time the API changes.
    pub async fn api_version(&mut self) -> Result<ApiVersionResponse> {
        self.get(&()).await
    }

    /// Creates a new session key
    pub async fn session_key(&mut self) -> Result<SessionKeyResponse> {
        self.get(&()).await
    }

    /// Verifies if the access key is valid and returns some information about its permissions
    pub async fn verify_access_key(&mut self) -> Result<VerifyAccessKeyResponse> {
        self.get(&()).await
    }

    /// Returns the list of tag and file services of the client
    pub async fn get_services(&mut self) -> Result<GetServicesResponse> {
        self.get(&()).await
    }
}
