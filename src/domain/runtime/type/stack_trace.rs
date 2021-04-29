/// See [StackTrace](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-StackTrace)

use serde::{Deserialize, Serialize};
use super::call_frame::CallFrame;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StackTrace {
    pub description: Option<String>,
    pub call_frames: Vec<CallFrame>,
}