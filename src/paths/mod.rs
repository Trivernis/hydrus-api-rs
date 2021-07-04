pub mod access_management;
pub mod common;

pub trait Path {
    fn get_path() -> String;
}