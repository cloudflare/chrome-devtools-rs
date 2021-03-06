//! [Runtime events](https://chromedevtools.github.io/devtools-protocol/tot/Runtime)

use serde::{Deserialize, Serialize};

pub mod r#type;

mod get_isolate_id;

use r#type::Id;

use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method")]
#[non_exhaustive]
pub enum SendMethod {
    #[serde(rename = "Runtime.enable")]
    Enable(SendId),
    #[serde(rename = "Runtime.disable")]
    Disable(SendId),
    #[serde(rename = "Runtime.getIsolateId")]
    GetIsolateId(SendId),
}

impl fmt::Display for SendMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method")]
#[non_exhaustive]
pub enum ReturnMethod {
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
