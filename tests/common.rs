use hydrus_api::api_core::client::Client;
use hydrus_api::Hydrus;
use std::env;
use std::sync::{Arc, Mutex, MutexGuard};

pub fn setup() {
    lazy_static::lazy_static! { static ref SETUP_DONE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false)); }
    let mut setup_done: MutexGuard<bool> = SETUP_DONE.lock().unwrap();

    if !*setup_done {
        dotenv::dotenv().expect("failed to initialize dotenv");
        tracing_subscriber::fmt::init();
        *setup_done = true;
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
