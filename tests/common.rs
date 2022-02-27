use hydrus_api::api_core::client::Client;
use hydrus_api::Hydrus;
use log::LevelFilter;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn setup() {
    lazy_static::lazy_static! { static ref SETUP_DONE: Arc<AtomicBool> = Arc::new(AtomicBool::new(false)); }
    if !SETUP_DONE.swap(true, Ordering::SeqCst) {
        dotenv::dotenv().expect("failed to initialize dotenv");
        env_logger::builder()
            .filter_level(LevelFilter::Trace)
            .init();
    }
}

pub fn get_client() -> Client {
    setup();

    Client::new(
        env::var("HYDRUS_URL").unwrap(),
        env::var("HYDRUS_ACCESS_KEY").unwrap(),
    )
}

pub fn get_hydrus() -> Hydrus {
    let client = get_client();

    Hydrus::new(client)
}
