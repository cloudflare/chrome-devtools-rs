//! An experimental new library that serializes and deserializes messages for the [Chrome Devtools
//! Protocol](https://chromedevtools.github.io/devtools-protocol/).
//!
//! The Devtools Protocol is divided into a number of domains. Each of these domains have "methods"
//! and "events" that can be both serialized and deserialized by this library.

pub mod domain;

pub use domain::{runtime, runtime::Runtime, Domain};
