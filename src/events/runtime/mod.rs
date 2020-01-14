pub mod console;
pub mod exception;
pub mod remote_object;

use console::ConsoleEvent;
use exception::ExceptionEvent;

use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
#[non_exhaustive]
pub enum RuntimeEvent {
    #[serde(rename = "Runtime.consoleAPICalled")]
    ConsoleAPICalled(ConsoleEvent),
    #[serde(rename = "Runtime.exceptionThrown")]
    ExceptionThrown(ExceptionEvent),
}

impl fmt::Display for RuntimeEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            RuntimeEvent::ConsoleAPICalled(event) => write!(f, "{}", event),
            RuntimeEvent::ExceptionThrown(event) => write!(f, "{}", event),
        }
    }
}
