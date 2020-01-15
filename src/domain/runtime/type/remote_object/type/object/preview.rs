use crate::domain::runtime::r#type::remote_object::r#type::object::{Entry, Property, Subtype};
use crate::domain::runtime::r#type::remote_object::Type;

use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    pub r#type: Type,
    pub description: String,
    pub overflow: bool,
    pub subtype: Option<Subtype>,
    pub properties: Vec<Property>,
    pub entries: Option<Vec<Entry>>,
}

impl Preview {
    fn parse_error(&self, f: &mut fmt::Formatter) -> fmt::Result {
        log::debug!("{:?}", &self);
        write!(f, "{{unknown ")?;
        if let Some(subtype) = &self.subtype {
            write!(f, "{}", subtype)?;
        } else {
            write!(f, "{}", &self.r#type)?;
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

impl fmt::Display for Preview {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(subtype) = &self.subtype {
            match subtype {
                Subtype::Array => {
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
                Subtype::Map => {
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
                Subtype::RegExp | Subtype::Date => write!(f, "{}", &self.description),
                _ => write!(f, "{}", subtype),
            }
        } else {
            match &self.r#type {
                r#Type::Object => {
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
                r#Type::String => write!(f, "\"{}\"", &self.description),
                _ => write!(f, "{}", &self.description),
            }
        }
    }
}
