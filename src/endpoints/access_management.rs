use crate::endpoints::common::BasicServiceInfo;
use crate::endpoints::Endpoint;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct ApiVersionResponse {
    pub version: u32,
    pub hydrus_version: u32,
}

pub struct ApiVersion;

impl Endpoint for ApiVersion {
    type Request = ();
    type Response = ApiVersionResponse;

    fn get_path() -> String {
        String::from("api_version")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SessionKeyResponse {
    pub session_key: String,
}

pub struct SessionKey;

impl Endpoint for SessionKey {
    type Request = ();
    type Response = SessionKeyResponse;

    fn get_path() -> String {
        String::from("session_key")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerifyAccessKeyResponse {
    pub basic_permissions: Vec<u32>,
    pub human_description: String,
}

pub struct VerifyAccessKey;

impl Endpoint for VerifyAccessKey {
    type Request = ();
    type Response = VerifyAccessKeyResponse;

    fn get_path() -> String {
        String::from("verify_access_key")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetServicesResponse(pub HashMap<String, Vec<BasicServiceInfo>>);

pub struct GetServices;

impl Endpoint for GetServices {
    type Request = ();
    type Response = GetServicesResponse;

    fn get_path() -> String {
        String::from("get_services")
    }
}
