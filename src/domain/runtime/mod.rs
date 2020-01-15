mod event;
mod method;
pub mod r#type;

pub use event::Event;
pub use method::{MethodReturn, MethodSend};

/// Runtime domain exposes JavaScript runtime by means of remote evaluation and mirror objects.
/// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Runtime {
    Event(Event),
    Method(MethodReturn),
}
