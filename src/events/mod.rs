mod console;
mod exception;

use console::ConsoleEvent;
use exception::ExceptionEvent;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum DevtoolsEvent {
    #[serde(rename = "Runtime.consoleAPICalled")]
    ConsoleAPICalled(ConsoleEvent),
    #[serde(rename = "Runtime.exceptionThrown")]
    ExceptionThrown(ExceptionEvent),
}
