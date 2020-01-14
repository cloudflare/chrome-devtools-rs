pub mod object;

use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RemoteObjectType {
    Object,
    Number,
    BigInt,
    Boolean,
    String,
    Symbol,
    Undefined,
    Function,
}

impl fmt::Display for RemoteObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let object_type = format!("{:?}", &self);
        write!(f, "{}", object_type.to_lowercase())
    }
}
