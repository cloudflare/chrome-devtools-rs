use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct MapData {
    pub preview: MapPreview,
    pub description: String,
}

impl fmt::Display for MapData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", &self.description, &self.preview)
        // write!(f, "{}", &self.description)
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
    pub key: MapMessage,
    pub value: MapMessage,
}

impl fmt::Display for MapEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}", &self.key, &self.value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapMessage {
    #[serde(rename = "type")]
    pub object_type: String,
    pub description: String,
}

impl fmt::Display for MapMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.description)
    }
}
