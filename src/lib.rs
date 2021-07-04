#[macro_use]
extern crate serde_derive;

pub mod client;
mod error;
pub mod paths;
pub(crate) mod utils;

pub use client::Client;
