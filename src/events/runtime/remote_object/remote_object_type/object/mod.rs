mod property;
mod subtype;

use std::fmt;

pub use property::ObjectProperty;
pub use subtype::ObjectSubtype;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectPreview {
    #[serde(rename = "type")]
    pub object_type: String,
    pub description: String,
    pub overflow: bool,
    pub subtype: Option<String>,
    pub properties: Vec<ObjectProperty>,
}

impl fmt::Display for ObjectPreview {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.properties.len();
        if len > 0 {
            let last_index = len - 1;
            for (idx, property) in &mut self.properties.iter().enumerate() {
                if idx == 0 {
                    write!(f, "{{")?;
                }
                write!(f, "{}", property)?;
                if idx < last_index {
                    write!(f, ", ")?;
                } else {
                    write!(f, "}}")?;
                }
            }
        } else {
            write!(f, "{{}}")?;
        }
        Ok(())
    }
}
