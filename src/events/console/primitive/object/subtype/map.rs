use std::fmt;

use crate::events::console::JsPrimitive;

#[derive(Debug, Serialize, Deserialize)]
pub struct MapData {
    pub preview: MapPreview,
    pub description: String,
}

impl fmt::Display for MapData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", &self.description, &self.preview)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapPreview {
    pub entries: Vec<MapEntry>,
}

impl fmt::Display for MapPreview {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        let last_index = self.entries.len() - 1;
        for (index, entry) in &mut self.entries.iter().enumerate() {
            write!(f, "{}", entry)?;
            if index < last_index {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapEntry {
    pub key: JsPrimitive,
    pub value: JsPrimitive,
}

impl fmt::Display for MapEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}", &self.key, &self.value)
    }
}
