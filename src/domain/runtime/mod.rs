pub mod event;
pub mod method;
pub mod r#type;

pub use event::Event;
pub use method::{MethodReturn, MethodSend};

use serde::{Deserialize, Serialize};

/// See documentation for the
/// [Runtime domain](https://chromedevtools.github.io/devtools-protocol/tot/Runtime)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Runtime {
    Event(Event),
    Method(MethodReturn),
}
