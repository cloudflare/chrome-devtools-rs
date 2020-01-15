pub mod r#type;

mod get_isolate_id;

use r#type::Id;

use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method")]
#[non_exhaustive]
pub enum MethodSend {
    #[serde(rename = "Runtime.enable")]
    Enable(SendId),
    #[serde(rename = "Runtime.disable")]
    Disable(SendId),
    #[serde(rename = "Runtime.getIsolateId")]
    GetIsolateId(SendId),
}

impl fmt::Display for MethodSend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method")]
#[non_exhaustive]
pub enum MethodReturn {
    #[serde(rename = "Runtime.enable")]
    Enable(Id),
    #[serde(rename = "Runtime.disable")]
    Disable(Id),
    #[serde(rename = "Runtime.getIsolateId")]
    GetIsolateId(get_isolate_id::Return),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendId {
    id: Id,
}

impl From<u64> for SendId {
    fn from(id: u64) -> Self {
        SendId { id }
    }
}
