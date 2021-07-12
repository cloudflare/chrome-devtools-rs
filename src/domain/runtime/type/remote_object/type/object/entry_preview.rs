use std::fmt;

use serde::{Deserialize, Serialize};

use crate::domain::runtime::r#type::remote_object::r#type::object::ObjectPreview;

/// See [EntryPreview](https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-EntryPreview)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntryPreview {
    pub key: ObjectPreview,
    pub value: ObjectPreview,
}

impl fmt::Display for EntryPreview {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}", &self.key, &self.value)
    }
}
