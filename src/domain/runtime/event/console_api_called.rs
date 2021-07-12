use std::fmt;

use serde::{Deserialize, Serialize};

use crate::domain::runtime::r#type::RemoteObject;

/// Issued when console API was called.
/// See [Runtime.consoleAPICalled](https://chromedevtools.github.io/devtools-protocol/tot/Runtime#event-bindingCalled)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    /// Type of the call
    pub r#type: String, // TODO: make this an enum
    /// Call arguments
    pub args: Vec<RemoteObject>,
    // TODO: Add other parameters
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let last_index = self.args.len() - 1;
        for (index, arg) in self.args.iter().enumerate() {
            write!(f, "{}", arg)?;
            if index < last_index {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}
