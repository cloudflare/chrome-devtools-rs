use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ObjectSubtype {
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

impl fmt::Display for ObjectSubtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let subtype = format!("{:?}", &self);
        write!(f, "{}", subtype.to_lowercase())
    }
}
