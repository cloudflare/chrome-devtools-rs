pub mod runtime;
pub use runtime::Runtime;

use serde::{Deserialize, Serialize};

/// `Domain` is an enum that should contain all of the different domains defined by the devtools
/// protocol. Currently only the `Runtime` domain is implemented, but this will hopefully change
/// as the library evolves.
///
/// It's important to note that this enum is tagged as `#[non_exhaustive]` because the plan is to
/// add more domains. This means that matching on a `Domain` will require handling a default `_`
/// case for the sake of future compatibility.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Domain {
    Runtime(Runtime),
}
