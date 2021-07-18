use crate::api_core::common::PageInformation;
use crate::api_core::Endpoint;

#[derive(Clone, Debug, Deserialize)]
pub struct GetPagesResponse {
    /// The top level notebook page
    pub pages: PageInformation,
}

pub struct GetPage;

impl Endpoint for GetPage {
    type Request = ();
    type Response = GetPagesResponse;

    fn path() -> String {
        String::from("manage_pages/get_pages")
    }
}
