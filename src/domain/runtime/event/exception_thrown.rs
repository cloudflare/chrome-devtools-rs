use std::fmt;

use console::style;
use serde::{Deserialize, Serialize};

use crate::domain::runtime::r#type::ExceptionDetails;

/// Issued when exception was thrown and unhandled.
/// See [Runtime.exceptionThrown](https://chromedevtools.github.io/devtools-protocol/tot/Runtime#event-exceptionThrown)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub exception_details: ExceptionDetails,
    // TODO: pub timestamp: Timestamp,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut disp = self.exception_details.text.to_string();

        let line_number = self.exception_details.line_number;
        let column_number = self.exception_details.column_number;

        if let Some(description) = &self.exception_details.exception.description {
            disp = format!(
                "{}\n{}\nat line: {} column: {}",
                disp, description, line_number, column_number
            );
        };
        if cfg!(feature = "color") {
            disp = format!("{}", style(disp).red())
        }
        write!(f, "{}", disp)
    }
}
