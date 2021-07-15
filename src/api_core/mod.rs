use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod access_management;
pub mod adding_files;
pub mod adding_tags;
pub mod adding_urls;
pub mod client;
pub mod common;
pub mod searching_and_fetching_files;

pub(crate) trait Endpoint {
    type Request: Serialize;
    type Response: DeserializeOwned;

    fn path() -> String;
}
