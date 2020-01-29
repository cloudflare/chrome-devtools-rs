use std::fmt;

use serde::{Deserialize, Serialize};

mod subtype;

pub use subtype::Subtype;

use crate::domain::runtime::r#type::remote_object::r#type::object::{
    EntryPreview, PropertyPreview,
};
use crate::domain::runtime::r#type::remote_object::Type;

/// See [ObjectPreview](https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-ObjectPreview)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectPreview {
    pub r#type: Type,
    pub description: String,
    pub overflow: bool,
    pub subtype: Option<Subtype>,
    pub properties: Vec<PropertyPreview>,
    pub entries: Option<Vec<EntryPreview>>,
}

impl ObjectPreview {
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

impl fmt::Display for ObjectPreview {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(subtype) = &self.subtype {
            match subtype {
                Subtype::Array => {
                    write!(f, "[ ")?;
                    let last_index = self.properties.len() - 1;
                    for (index, property) in &mut self.properties.iter().enumerate() {
                        write!(f, "{}", property.get_value())?;
                        if index < last_index {
                            write!(f, ", ")?;
                        }
                    }
                    self.write_overflow(f)?;
                    write!(f, " ]")
                }
                Subtype::Map => {
                    if let Some(entries) = &self.entries {
                        write!(f, "{{ ")?;
                        let last_index = entries.len() - 1;
                        for (index, entry) in &mut entries.iter().enumerate() {
                            write!(f, "{}", entry)?;
                            if index < last_index {
                                write!(f, ", ")?;
                            }
                        }
                        self.write_overflow(f)?;
                        write!(f, " }}")
                    } else {
                        self.parse_error(f)
                    }
                }
                Subtype::RegExp | Subtype::Date => write!(f, "{}", &self.description),
                _ => write!(f, "{}", subtype),
            }
        } else {
            match &self.r#type {
                Type::Object => {
                    write!(f, "{{ ")?;
                    let last_index = self.properties.len() - 1;
                    for (idx, property) in &mut self.properties.iter().enumerate() {
                        write!(f, "{}", property)?;
                        if idx < last_index {
                            write!(f, ", ")?;
                        }
                    }
                    self.write_overflow(f)?;
                    write!(f, " }}")
                }
                Type::String => write!(f, "\"{}\"", &self.description),
                _ => write!(f, "{}", &self.description),
            }
        }
    }
}
