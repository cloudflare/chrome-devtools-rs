pub mod entry;
pub mod property;
pub mod subtype;

use std::fmt;

pub use entry::ObjectEntry;
pub use property::ObjectProperty;
pub use subtype::ObjectSubtype;

use super::RemoteObjectType;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectPreview {
    #[serde(rename = "type")]
    pub object_type: RemoteObjectType,
    pub description: String,
    pub overflow: bool,
    pub subtype: Option<ObjectSubtype>,
    pub properties: Vec<ObjectProperty>,
    pub entries: Option<Vec<ObjectEntry>>,
}

impl ObjectPreview {
    fn parse_error(&self, f: &mut fmt::Formatter) -> fmt::Result {
        log::debug!("{:?}", &self);
        write!(f, "{{unknown ")?;
        if let Some(subtype) = &self.subtype {
            write!(f, "{}", subtype)?;
        } else {
            write!(f, "{}", &self.object_type)?;
        }
        write!(f, "}}")
    }

    fn write_overflow(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.overflow {
            write!(f, ", …")
        } else {
            Ok(())
        }
    }
}

impl fmt::Display for ObjectPreview {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(subtype) = &self.subtype {
            match subtype {
                ObjectSubtype::Array => {
                    write!(f, " [")?;
                    let last_index = self.properties.len() - 1;
                    for (index, property) in &mut self.properties.iter().enumerate() {
                        if property.object_type.as_str() == "string" {
                            if let Some(value) = &property.value {
                                write!(f, "\"{}\"", value)?;
                            } else {
                                self.parse_error(f)?;
                            }
                        } else {
                            write!(f, "{}", property)?;
                        }
                        if index < last_index {
                            write!(f, ", ")?;
                        }
                    }
                    self.write_overflow(f)?;
                    write!(f, "]")
                }
                ObjectSubtype::Map => {
                    if let Some(entries) = &self.entries {
                        write!(f, "{{")?;
                        let last_index = entries.len() - 1;
                        for (index, entry) in &mut entries.iter().enumerate() {
                            write!(f, "{}", entry)?;
                            if index < last_index {
                                write!(f, ", ")?;
                            }
                        }
                        self.write_overflow(f)?;
                        write!(f, "}}")
                    } else {
                        self.parse_error(f)
                    }
                }
                ObjectSubtype::RegExp | ObjectSubtype::Date => write!(f, "{}", &self.description),
                _ => write!(f, "{}", subtype),
            }
        } else {
            match &self.object_type {
                RemoteObjectType::Object => {
                    write!(f, "{{")?;
                    let last_index = self.properties.len() - 1;
                    for (idx, property) in &mut self.properties.iter().enumerate() {
                        write!(f, "{}", property)?;
                        if idx < last_index {
                            write!(f, ", ")?;
                        }
                    }
                    self.write_overflow(f)?;
                    write!(f, "}}")
                }
                RemoteObjectType::String => write!(f, "\"{}\"", &self.description),
                _ => write!(f, "{}", &self.description),
            }
        }
    }
}
