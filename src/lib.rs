#![crate_name = "chrome_devtools"]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod domain;

pub use domain::Domain;
