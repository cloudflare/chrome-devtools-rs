pub mod runtime;

pub use runtime::RuntimeEvent;

use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum DevtoolsEvent {
    Runtime(RuntimeEvent),
}

impl fmt::Display for DevtoolsEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            DevtoolsEvent::Runtime(event) => write!(f, "{}", event),
        }
    }
}
