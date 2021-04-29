//![Runtime events](https://chromedevtools.github.io/devtools-protocol/tot/Runtime)

use crate::domain::runtime::r#type::RemoteObject;
use crate::domain::runtime::r#type::StackTrace;
use serde::{Deserialize, Serialize};
use std::fmt;
pub mod console_api_called;

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "method", content = "params")]
pub enum Event {
    #[serde(rename = "Runtime.consoleAPICalled")]
    ConsoleAPICalled(console_api_called::Event),
    #[serde(rename = "Runtime.exceptionThrown")]
    ExceptionThrown(ExceptionParams),
}

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct ExceptionParams {
    pub timestamp: i32,
    pub exception_details: ExceptionDetails,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionDetails {
    pub exception_id: i32,
    pub text: String,
    pub line_number: i32,
    pub column_number: i32,
    pub url: Option<String>,
    pub exception: RemoteObject, // optional in spec
    pub stack_trace: StackTrace, // ditto
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Event::ConsoleAPICalled(event) => write!(f, "{}", event),
            Event::ExceptionThrown(params) => params.exception_details.text.fmt(f),
        }
    }
}
