use std::fmt;

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
        write!(f, "{}", &self.exception_details.text)?;
        if let Some(description) = &self.exception_details.exception.description {
            write!(f, "\n{}", description)?;
        };
        Ok(())
    }
}
