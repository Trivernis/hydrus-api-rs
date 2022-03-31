use hydrus_api::api_core::client::Client;
use hydrus_api::api_core::endpoints::adding_urls::AddUrlRequestBuilder;
use hydrus_api::Hydrus;
use std::env;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;
use test_data::TEST_URLS;
pub mod test_data;

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
    Client::builder()
        .url(env::var("HYDRUS_URL").unwrap())
        .access_key(env::var("HYDRUS_ACCESS_KEY").unwrap())
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

pub fn get_hydrus() -> Hydrus {
    let client = get_client();

    Hydrus::new(client)
}

pub async fn create_testdata(client: &Client) {
    for url in TEST_URLS {
        client
            .add_url(AddUrlRequestBuilder::default().url(url).build())
            .await
            .unwrap();
    }
}
