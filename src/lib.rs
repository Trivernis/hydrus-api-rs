//! ## About
//! This crate provides a rust wrapper for the hydrus client API. All request require an access
//! token that can be retrieved in the hydrus client from the *review services* dialog.
//! Different actions require different permissions, you can read about it in the [official docs](https://hydrusnetwork.github.io/hydrus/help/client_api.html).
//!
//! ## Hydrus Usage Example
//!
//! ```
//! # use hydrus_api::{Hydrus, Client};
//! use std::env;
//! use hydrus_api::wrapper::tag::Tag;
//! use hydrus_api::wrapper::service::ServiceName;
//! use hydrus_api::wrapper::hydrus_file::FileStatus;
//! use hydrus_api::wrapper::page::PageIdentifier;
//! use hydrus_api::wrapper::builders::tag_builder::{SystemTagBuilder, Comparator};
//!
//! # #[tokio::test]
//! # async fn doctest() {
//! let hydrus_url = env::var("HYDRUS_URL").unwrap();
//! let access_key = env::var("HYDRUS_ACCESS_KEY").unwrap();
//! let hydrus = Hydrus::new(Client::new(hydrus_url, access_key));
//! let files = hydrus.search(vec![
//!     Tag::from("character:megumin"),
//!     SystemTagBuilder::new().archive().build(),
//!     SystemTagBuilder::new().tag_namespace_as_number("page", Comparator::Equal, 5).negate().build(),
//! ]).await.unwrap();
//!
//! for mut file in files {
//!     file.add_tags(ServiceName::my_tags(), vec![Tag::from("ark mage")]).await.unwrap();
//! }
//!
//! let url = hydrus.import()
//!     .url("https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium")
//!     .page(PageIdentifier::name("My Import Page"))
//!     .add_additional_tag(ServiceName::my_tags(), Tag::from("character:megumin"))
//!     .show_page(true)
//!     .run().await.unwrap();
//! # }
//! ```
//!
//! ## Client Usage Example
//! ```
//! use hydrus_api::Client;
//! use hydrus_api::api_core::adding_tags::{AddTagsRequestBuilder, TagAction};
//! use std::env;
//! # #[tokio::test]
//! # async fn doctest() {
//!
//! Client::new(
//!     env::var("HYDRUS_URL").unwrap(),
//!     env::var("HYDRUS_ACCESS_KEY").unwrap(),
//! );
//! // let's first import a file
//! let hash = client.add_file("/path/to/my/file").await.unwrap().hash;
//!
//! // and now let's add tags to it
//! let request = AddTagsRequestBuilder::default()
//!     .add_hash(hash)
//!     // for each tag the service has to be specified
//!     .add_tags("my tags", vec!["beach".into(), "summer".into()])
//!     // with tag actions tags can also be removed. It's especially useful for the PTR
//!     .add_tag_with_action("my tags", "rain", TagAction::DeleteFromLocalService)
//!     .build();
//!
//! client.add_tags(request).await.unwrap();
//! # }
//! ```

#[macro_use]
extern crate serde;

pub use api_core::client::Client;
pub use wrapper::hydrus::Hydrus;

pub mod api_core;
pub mod error;
pub(crate) mod utils;
pub mod wrapper;
