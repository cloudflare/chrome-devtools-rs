use std::fmt;

pub mod r#type;

use r#type::object::{ObjectPreview, Subtype};
use r#type::Type;

use serde::{Deserialize, Serialize};
use serde_json;

/// Mirror object referencing original JavaScript object.
/// See [RemoteObject](https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-RemoteObject)
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteObject {
    pub r#type: Type,
    pub subtype: Option<Subtype>,
    pub description: Option<String>,
    pub class_name: Option<String>,
    pub value: Option<serde_json::Value>,
    pub preview: Option<ObjectPreview>,
}

impl RemoteObject {
    fn parse_error(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{unknown ")?;
        if let Some(subtype) = &self.subtype {
            write!(f, "{}", subtype)?;
        } else {
            write!(f, "{}", &self.r#type)?;
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
        if let Some(preview) = &self.preview {
            write!(f, "{}", preview)
        } else if let Some(subtype) = &self.subtype {
            if subtype.to_string() == "null" {
                write!(f, "null")
            } else {
                self.parse_error(f)
            }
        } else {
            self.parse_error(f)
        }
    }
}

impl fmt::Display for RemoteObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.r#type {
            Type::Undefined => write!(f, "undefined"),
            Type::Boolean | Type::String => self.display_value(f),
            Type::Object => self.display_object(f),
            Type::BigInt | Type::Number | Type::Symbol | Type::Function => {
                self.display_description(f)
            }
        }
    }
}
