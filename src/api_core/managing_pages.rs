use crate::api_core::common::PageInformation;
use crate::api_core::Endpoint;

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
