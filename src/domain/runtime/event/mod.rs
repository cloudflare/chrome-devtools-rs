//![Runtime events](https://chromedevtools.github.io/devtools-protocol/tot/Runtime)

use std::fmt;

use serde::{Deserialize, Serialize};

pub mod console_api_called;
pub mod exception_thrown;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
#[non_exhaustive]
pub enum Event {
    #[serde(rename = "Runtime.consoleAPICalled")]
    ConsoleAPICalled(console_api_called::Event),
    #[serde(rename = "Runtime.exceptionThrown")]
    ExceptionThrown(exception_thrown::Event),
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Event::ConsoleAPICalled(event) => write!(f, "{}", event),
            Event::ExceptionThrown(event) => write!(f, "{}", event),
        }
    }
}
