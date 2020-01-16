use serde::{Deserialize, Serialize};

/// Detailed information about exception (or error) that was thrown during script compilation or execution.
/// See [ExceptionDetails](https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-ExceptionDetails)
#[derive(Debug, Serialize, Deserialize)]
pub struct ExceptionDetails {
    pub text: String,
    pub exception: Exception,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exception {
    pub description: Option<String>,
}
