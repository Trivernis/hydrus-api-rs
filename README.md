# Hydrus Rust API

This is a WIP Rust Wrapper for the Hydrus Client API. 
The official API documentation can be found [here](https://hydrusnetwork.github.io/hydrus/help/client_api.html).

## Example

```rust

use hydrus_api_rs::Client;

#[tokio::main]
async fn main() {
    Client::new(
        env::var("HYDRUS_URL").unwrap(),
        env::var("HYDRUS_ACCESS_KEY").unwrap(),
    ).unwrap();
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