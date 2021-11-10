use std::fmt;

use console::style;
use serde::{Deserialize, Serialize};

mod subtype;

pub use subtype::Subtype;

use crate::domain::runtime::r#type::remote_object::r#type::object::{
    EntryPreview, PropertyPreview,
};
use crate::domain::runtime::r#type::remote_object::Type;

/// See [ObjectPreview](https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-ObjectPreview)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectPreview {
    pub r#type: Type,
    pub description: String,
    pub overflow: bool,
    pub subtype: Option<Subtype>,
    pub properties: Vec<PropertyPreview>,
    pub entries: Option<Vec<EntryPreview>>,
}

impl fmt::Display for ObjectPreview {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let disp = if let Some(subtype) = &self.subtype {
            match subtype {
                Subtype::Array => {
                    let last_index = self.properties.len().saturating_sub(1);
                    let mut array = "".to_string();
                    for (index, property) in &mut self.properties.iter().enumerate() {
                        if index == 0 {
                            array += " ";
                        }
                        array += &property.get_value();
                        if index < last_index {
                            array += ", ";
                        } else if self.overflow {
                            array += ", …";
                        }
                        array += " ";
                    }
                    format!("[{}]", array)
                }
                Subtype::Map => {
                    let map = if let Some(entries) = &self.entries {
                        let last_index = entries.len().saturating_sub(1);
                        let mut map = "".to_string();
                        for (index, entry) in &mut entries.iter().enumerate() {
                            if index == 0 {
                                map += " ";
                            }
                            map += &format!("{}", entry);
                            if index < last_index {
                                map += ", ";
                            } else if self.overflow {
                                map += ", …"
                            }
                        }
                        map += " ";
                        map
                    } else {
                        "".to_string()
                    };
                    format!("Map {{{}}}", map)
                }
                Subtype::RegExp => {
                    let mut disp = self.description.to_string();
                    if cfg!(feature = "color") {
                        disp = format!("{}", style(disp).red())
                    }
                    disp
                }
                Subtype::Date => {
                    let mut disp = self.description.to_string();
                    if cfg!(feature = "color") {
                        disp = format!("{}", style(disp).magenta())
                    }
                    disp
                }
                _ => format!("{}", subtype),
            }
        } else {
            match &self.r#type {
                Type::Object => {
                    let last_index = self.properties.len().saturating_sub(1);
                    let mut object = "".to_string();
                    for (idx, property) in &mut self.properties.iter().enumerate() {
                        if idx == 0 {
                            object += " ";
                        }
                        object += &format!("{}", property);
                        if idx < last_index {
                            object += ",";
                        } else if self.overflow {
                            object += ", …";
                        }
                        object += " ";
                    }
                    format!("{{{}}}", object)
                }
                Type::String => {
                    if cfg!(feature = "color") {
                        format!("'{}'", style(&self.description).green())
                    } else {
                        format!("'{}'", &self.description)
                    }
                }
                _ => self.description.to_string(),
            }
        };
        write!(f, "{}", disp)
    }
}
