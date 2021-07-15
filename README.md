<h1 align="center">
Hydrus Rust API
</h1>
<p align="center">
    <a href="https://crates.io/crates/hydrus-api">
        <img src="https://img.shields.io/crates/v/hydrus-api?style=for-the-badge">
    </a>
    <a href="https://docs.rs/hydrus-api">
        <img src="https://img.shields.io/docsrs/hydrus-api?style=for-the-badge">
    </a>
</p>


This is a WIP Rust Wrapper for the Hydrus Client API. 
The official API documentation can be found [here](https://hydrusnetwork.github.io/hydrus/help/client_api.html).

## Example with Wrapper

```rust
use std::env;
use hydrus_api::api_core::searching_and_fetching_files::FileSearchLocation;
use hydrus_api::wrapper::tag::Tag;
use hydrus_api::wrapper::service::ServiceName;
use hydrus_api::wrapper::hydrus_file::FileStatus;
use hydrus_api::wrapper::page::PageIdentifier;

#[tokio::main]
async fn main() {
    let hydrus_url = env::var("HYDRUS_URL").unwrap();
    let access_key = env::var("HYDRUS_ACCESS_KEY").unwrap();
    
    let hydrus = Hydrus::new(Client::new(hydrus_url, access_key));
    let files = hydrus.search(FileSearchLocation::Archive,vec![Tag::from("character:megumin")]).await.unwrap();

    for mut file in files {
        file.add_tags(ServiceName::my_tags(), vec![Tag::from("ark mage")]).await.unwrap();
    }

    let url = hydrus.import()
        .url("https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium")
        .page(PageIdentifier::name("My Import Page"))
        .add_additional_tag(ServiceName::my_tags(), Tag::from("character:megumin"))
        .show_page(true)
        .run().await.unwrap();
}
```

## Example with Client

```rust
use hydrus_api::Client;
use hydrus_api::paths::adding_tags::{AddTagsRequestBuilder, TagAction};
use std::env;

#[tokio::main]
async fn main() {
    Client::new(
        env::var("HYDRUS_URL").unwrap(),
        env::var("HYDRUS_ACCESS_KEY").unwrap(),
    );
    // let's first import a file
    let hash = client.add_file("/path/to/my/file").await.unwrap().hash;
    
    // and now let's add tags to it
    let request = AddTagsRequestBuilder::default()
        .add_hash(hash)
        // for each tag the service has to be specified
        .add_tags("my tags", vec!["beach".into(), "summer".into()])
        // with tag actions tags can also be removed. It's especially useful for the PTR
        .add_tag_with_action("my tags", "rain", TagAction::DeleteFromLocalService)
        .build();
    
    client.add_tags(request).await.unwrap();
}
```

## License

Apache-2.0