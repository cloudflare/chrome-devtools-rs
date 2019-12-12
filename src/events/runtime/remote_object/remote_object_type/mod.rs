pub mod object;

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
    Null,
}
