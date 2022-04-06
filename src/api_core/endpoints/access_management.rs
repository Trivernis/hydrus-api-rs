use crate::api_core::common::BasicServiceInfo;
use crate::api_core::endpoints::Endpoint;
use std::collections::HashMap;

pub static SERVICE_TYPE_LOCAL_TAGS: &str = "local_tags";
pub static SERVICE_TYPE_TAG_REPOSITORIES: &str = "tag_repositories";
pub static SERVICE_TYPE_LOCAL_FILES: &str = "local_files";
pub static SERVICE_TYPE_FILE_REPOSITORIES: &str = "file_repositories";
pub static SERVICE_TYPE_ALL_LOCAL_FILES: &str = "all_local_files";
pub static SERVICE_TYPE_ALL_KNOWN_FILES: &str = "all_known_files";
pub static SERVICE_TYPE_ALL_KNOWN_TAGS: &str = "all_known_tags";
pub static SERVICE_TYPE_TRASH: &str = "trash";

#[derive(Debug, Clone, Deserialize)]
pub struct ApiVersionResponse {
    pub version: u32,
    pub hydrus_version: u32,
}

pub struct ApiVersion;

impl Endpoint for ApiVersion {
    type Request = ();
    type Response = ApiVersionResponse;

    fn path() -> String {
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

    fn path() -> String {
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

    fn path() -> String {
        String::from("verify_access_key")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetServicesResponse(pub HashMap<String, Vec<BasicServiceInfo>>);

pub struct GetServices;

impl Endpoint for GetServices {
    type Request = ();
    type Response = GetServicesResponse;

    fn path() -> String {
        String::from("get_services")
    }
}
