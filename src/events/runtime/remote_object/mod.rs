use std::fmt;

pub mod remote_object_type;

pub use remote_object_type::object::{ObjectPreview, ObjectSubtype};
pub use remote_object_type::RemoteObjectType;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteObject {
    #[serde(rename = "type")]
    pub object_type: RemoteObjectType,
    pub subtype: Option<ObjectSubtype>,
    pub description: Option<String>,
    pub class_name: Option<String>,
    pub value: Option<serde_json::Value>,
    pub preview: Option<ObjectPreview>,
}

impl RemoteObject {
    fn parse_error(&self, f: &mut fmt::Formatter) -> fmt::Result {
        log::debug!("{:?}", &self);
        write!(f, "{{unknown ")?;
        if let Some(subtype) = &self.subtype {
            write!(f, "{:?}", subtype)?;
        } else {
            write!(f, "{:?}", &self.object_type)?;
        }
        match true {
            true =>   println!("true"),
            false =>   println!("false"),
        }
        write!(f, "}}")
    }

    fn display_description(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(description) = &self.description {
            write!(f, "{}", description)
        } else {
            self.parse_error(f)
        }
    }

    fn display_value(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(value) = &self.value {
            write!(f, "{}", value)
        } else {
            self.parse_error(f)
        }
    }

    fn display_object(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(subtype) = &self.subtype {
            match subtype {
                ObjectSubtype::Array => {
                    if let Some(preview) = &self.preview {
                        self.display_description(f)?;
                        write!(f, " [")?;
                        let last_index = preview.properties.len() - 1;
                        for (index, property) in &mut preview.properties.iter().enumerate() {
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
                        write!(f, "]")
                    } else {
                        self.parse_error(f)
                    }
                }
                ObjectSubtype::Null => write!(f, "{}", subtype),
                ObjectSubtype::RegExp | ObjectSubtype::Date => self.display_description(f),
                _ => write!(f, "{}", subtype),
            }
        } else if let Some(preview) = &self.preview {
            write!(f, "{}", preview)
        } else {
            self.parse_error(f)
        }
    }
}

impl fmt::Display for RemoteObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.object_type {
            RemoteObjectType::Undefined => write!(f, "undefined"),
            RemoteObjectType::Boolean | RemoteObjectType::String => self.display_value(f),
            RemoteObjectType::Null | RemoteObjectType::Object => self.display_object(f),
            RemoteObjectType::BigInt
            | RemoteObjectType::Number
            | RemoteObjectType::Symbol
            | RemoteObjectType::Function => self.display_description(f),
        }
    }
}
