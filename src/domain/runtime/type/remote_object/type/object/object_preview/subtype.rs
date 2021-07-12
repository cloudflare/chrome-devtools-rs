use std::fmt;

use serde::{Deserialize, Serialize};

/// Object subtype hint. Specified for the `object` [Type][crate::domain::runtime::type::remote_object::type::Type] values only
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Subtype {
    Array,
    Null,
    RegExp,
    Date,
    Map,
    Node,
    Set,
    WeakMap,
    WeakSet,
    Iterator,
    Generator,
    Error,
    Proxy,
    Promise,
    TypedArray,
    ArrayBuffer,
    DataView,
}

impl fmt::Display for Subtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let subtype = format!("{:?}", &self);
        write!(f, "{}", subtype.to_lowercase())
    }
}
