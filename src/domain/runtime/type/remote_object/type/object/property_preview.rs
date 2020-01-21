use std::fmt;

use serde::{Deserialize, Serialize};

use crate::runtime::r#type::remote_object::r#type::object::object_preview::{
    ObjectPreview, Subtype,
};
use crate::runtime::r#type::remote_object::r#type::Type;

/// See [PropertyPreview](https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-PropertyPreview)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyPreview {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: Type,
    pub value: Option<String>,
    pub value_preview: Option<ObjectPreview>,
    pub subtype: Option<Subtype>,
}

impl PropertyPreview {
    pub fn get_value(&self) -> String {
        if let Some(value) = &self.value {
            match &self.r#type {
                Type::Undefined => "undefined".to_string(),
                Type::String => format!("'{}'", value),
                Type::Object => {
                    let r#type = if let Some(subtype) = &self.subtype {
                        format!("{:?}", subtype)
                    } else {
                        "Object".to_string()
                    };
                    format!("[object {}]", r#type)
                }
                Type::BigInt | Type::Number | Type::Symbol | Type::Function | Type::Boolean => {
                    value.to_string()
                }
            }
        } else {
            format!("[{} {}]", &self.r#type, &self.name)
        }
    }
}

impl fmt::Display for PropertyPreview {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = &self.get_value();
        write!(f, "{}: {}", &self.name, value)
    }
}
