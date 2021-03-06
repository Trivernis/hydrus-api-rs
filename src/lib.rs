//! ## About
//! This crate provides a rust wrapper for the hydrus client API. All request require an access
//! token that can be retrieved in the hydrus client from the *review services* dialog.
//! Different actions require different permissions, you can read about it in the [official docs](https://hydrusnetwork.github.io/hydrus/help/client_api.html).
//!
//! Starting with hydrus version 477, CBOR can be used as an alternative to JSON.
//! CBOR support can be enabled with the `cbor` feature of this crate. This feature is
//! incompatible with the `json` feature which is enabled by default.
//!
//! ## Hydrus Usage Example
//!
//! ```
//! use hydrus_api::{Hydrus, Client};
//! use std::env;
//! use hydrus_api::wrapper::tag::Tag;
//! use hydrus_api::wrapper::service::ServiceName;
//! use hydrus_api::wrapper::hydrus_file::FileStatus;
//! use hydrus_api::wrapper::page::PageIdentifier;
//! use hydrus_api::wrapper::builders::tag_builder::{SystemTagBuilder, Comparator};
//! use hydrus_api::wrapper::builders::search_builder::SortType;
//! use hydrus_api::wrapper::builders::or_chain_builder::OrChainBuilder;
//!
//! # #[tokio::test]
//! # async fn doctest() {
//! let hydrus_url = env::var("HYDRUS_URL").unwrap();
//! let access_key = env::var("HYDRUS_ACCESS_KEY").unwrap();
//! let hydrus = Hydrus::new(Client::new(hydrus_url, access_key));
//! let files = hydrus.search()
//!     .add_tag(Tag::from("character:megumin"))
//!     .add_tag(SystemTagBuilder::new().archive().build())
//!     .add_tag(SystemTagBuilder::new()
//!         .tag_namespace_as_number("page", Comparator::Equal, 5).negate().build())
//!     .add_or_chain(
//!         OrChainBuilder::new()
//!             .add_tag("summer".into())
//!             .add_tag("winter".into())
//!             .build(),
//!     )
//!     .sort_by(SortType::NumberOfPixels)
//!     .sort_descending()
//!     .run().await.unwrap();
//!
//! for mut file in files {
//!     file.add_tags(ServiceName::my_tags().into(), vec![Tag::from("ark mage")]).await.unwrap();
//! }
//!
//! let url = hydrus.import()
//!     .url("https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium")
//!     .page(PageIdentifier::name("My Import Page"))
//!     .add_additional_tag(ServiceName::my_tags().into(), Tag::from("character:megumin"))
//!     .show_page(true)
//!     .run().await.unwrap();
//! # }
//! ```
//!
//! ## Client Usage Example
//! ```
//! use hydrus_api::Client;
//! use hydrus_api::api_core::endpoints::adding_tags::{AddTagsRequestBuilder, TagAction};
//! use std::env;
//! use hydrus_api::api_core::common::ServiceIdentifier;
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
//!     .add_tags(ServiceIdentifier::name("my tags"), vec!["beach".into(), "summer".into()])
//!     // with tag actions tags can also be removed. It's especially useful for the PTR
//!     .add_tag_with_action(ServiceIdentifier::name("my tags"), "rain", TagAction::DeleteFromLocalService)
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
pub mod utils;
pub mod wrapper;

#[cfg(all(feature = "cbor", feature = "json"))]
compile_error!("Feature 'cbor' and 'json' cannot be enabled at the same time");

#[cfg(not(any(feature = "cbor", feature = "json")))]
compile_error!("Either the 'json' or 'cbor' feature must be selected.");
