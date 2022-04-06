use crate::api_core::common::PageInformation;
use crate::api_core::endpoints::Endpoint;

#[derive(Clone, Debug, Deserialize)]
pub struct GetPagesResponse {
    /// The top level notebook page
    pub pages: PageInformation,
}

pub struct GetPages;

impl Endpoint for GetPages {
    type Request = ();
    type Response = GetPagesResponse;

    fn path() -> String {
        String::from("manage_pages/get_pages")
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetPageInfoResponse {
    pub page_info: PageInformation,
}

pub struct GetPageInfo;

impl Endpoint for GetPageInfo {
    type Request = ();
    type Response = GetPageInfoResponse;

    fn path() -> String {
        String::from("manage_pages/get_page_info")
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct FocusPageRequest {
    pub page_key: String,
}

pub struct FocusPage;

impl Endpoint for FocusPage {
    type Request = FocusPageRequest;
    type Response = ();

    fn path() -> String {
        String::from("manage_pages/focus_page")
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct AddFilesRequest {
    pub page_key: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_ids: Vec<u64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hashes: Vec<String>,
}

pub struct AddFiles;

impl Endpoint for AddFiles {
    type Request = AddFilesRequest;
    type Response = ();

    fn path() -> String {
        String::from("manage_pages/add_files")
    }
}
