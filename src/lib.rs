//! ## About
//! This crate provides a rust wrapper for the hydrus client API. All request require an access
//! token that can be retrieved in the hydrus client from the *review services* dialog.
//! Different actions require different permissions, you can read about it in the [official docs](https://hydrusnetwork.github.io/hydrus/help/client_api.html).
//!
//! ## Usage Example
//! ```
//! use hydrus_api::Client;
//! use hydrus_api::endpoints::adding_tags::{AddTagsRequestBuilder, TagAction};
//! use std::env;
//! # #[tokio::test]
//! # async fn doctest() {
//!
//! Client::new(
//!     env::var("HYDRUS_URL").unwrap(),
//!     env::var("HYDRUS_ACCESS_KEY").unwrap(),
//! ).unwrap();
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
extern crate serde_derive;

pub mod client;
pub mod endpoints;
pub mod error;
pub(crate) mod utils;

pub use client::Client;
