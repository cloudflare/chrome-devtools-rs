pub mod object;

use std::fmt;

use serde::{Deserialize, Serialize};

/// Object type.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Object,
    Number,
    BigInt,
    Boolean,
    String,
    Symbol,
    Undefined,
    Function,
    Accessor,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r#type = format!("{:?}", &self);
        write!(f, "{}", r#type.to_lowercase())
    }
}
