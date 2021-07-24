use super::super::common;
use hydrus_api::wrapper::address::{Address, DomainCookie};
use std::time::{Duration, SystemTime};

fn get_address() -> Address {
    let hydrus = common::get_hydrus();

    hydrus.address("trivernis.net/some/path")
}

#[tokio::test]
async fn it_sets_cookies() {
    let address = get_address();
    address
        .set_cookies(vec![
            DomainCookie::new("name", "value", None),
            DomainCookie::new(
                "name2",
                "value2",
                Some(SystemTime::now() + Duration::from_secs(30)),
            ),
        ])
        .await
        .unwrap();
}

#[tokio::test]
async fn it_retrieves_cookies() {
    let address = get_address();
    address.get_cookies().await.unwrap();
}
