use crate::paths::common::BasicServiceInfo;
use crate::paths::Path;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiVersionResponse {
    pub version: u32,
    pub hydrus_version: u32,
}

impl Path for ApiVersionResponse {
    fn get_path() -> String {
        String::from("api_version")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionKeyResponse {
    pub session_key: String,
}

impl Path for SessionKeyResponse {
    fn get_path() -> String {
        String::from("session_key")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyAccessKeyResponse {
    pub basic_permissions: Vec<u32>,
    pub human_description: String,
}

impl Path for VerifyAccessKeyResponse {
    fn get_path() -> String {
        String::from("verify_access_key")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetServicesResponse(pub HashMap<String, Vec<BasicServiceInfo>>);

impl Path for GetServicesResponse {
    fn get_path() -> String {
        String::from("get_services")
    }
}
