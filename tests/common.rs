use hydrus_api::client::Client;
use std::env;

pub fn get_client() -> Client {
    Client::new(env::var("HYDRUS_URL").unwrap(), env::var("HYDRUS_ACCESS_KEY").unwrap()).unwrap()
}