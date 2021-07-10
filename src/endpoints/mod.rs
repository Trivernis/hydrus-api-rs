use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod access_management;
pub mod adding_files;
pub mod adding_tags;
pub mod common;

pub trait Endpoint {
    type Request: Serialize;
    type Response: DeserializeOwned;

    fn get_path() -> String;
}
