mod console;
mod exception;

pub use console::ConsoleEvent;
pub use exception::ExceptionEvent;

use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum DevtoolsEvent {
    #[serde(rename = "Runtime.consoleAPICalled")]
    ConsoleAPICalled(ConsoleEvent),
    #[serde(rename = "Runtime.exceptionThrown")]
    ExceptionThrown(ExceptionEvent),
}

impl fmt::Display for DevtoolsEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            DevtoolsEvent::ConsoleAPICalled(event) => write!(f, "{}", event),
            DevtoolsEvent::ExceptionThrown(event) => write!(f, "{}", event),
        }
    }
}
