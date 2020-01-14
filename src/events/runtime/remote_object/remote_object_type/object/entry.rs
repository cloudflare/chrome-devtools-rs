use std::fmt;

use super::ObjectPreview;

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectEntry {
    pub key: ObjectPreview,
    pub value: ObjectPreview,
}

impl fmt::Display for ObjectEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}", &self.key, &self.value)
    }
}
