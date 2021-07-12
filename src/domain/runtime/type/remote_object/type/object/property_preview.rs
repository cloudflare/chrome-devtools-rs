use std::fmt;

use console::style;
use serde::{Deserialize, Serialize};

use crate::runtime::r#type::remote_object::r#type::object::object_preview::{
    ObjectPreview, Subtype,
};
use crate::runtime::r#type::remote_object::r#type::Type;

/// See [PropertyPreview](https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-PropertyPreview)
#[derive(Debug, Deserialize, Serialize, Clone)]
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
                Type::Undefined => {
                    let mut value = "undefined".to_string();
                    if cfg!(feature = "color") {
                        value = format!("{}", style(value).dim());
                    }
                    value
                }
                Type::String => {
                    let mut value = format!("'{}'", value);
                    if cfg!(feature = "color") {
                        value = format!("{}", style(value).green());
                    }
                    value
                }
                Type::Object => {
                    let r#type = if let Some(subtype) = &self.subtype {
                        format!("{:?}", subtype)
                    } else {
                        "Object".to_string()
                    };
                    let mut value = format!("[{}]", r#type);
                    if cfg!(feature = "color") {
                        value = format!("{}", style(value).cyan())
                    }
                    value
                }
                Type::BigInt | Type::Number | Type::Boolean => {
                    if cfg!(feature = "color") {
                        format!("{}", style(value).yellow())
                    } else {
                        value.to_string()
                    }
                }
                Type::Symbol => {
                    if cfg!(feature = "color") {
                        format!("{}", style(value).green())
                    } else {
                        value.to_string()
                    }
                }
                Type::Function => {
                    if cfg!(feature = "color") {
                        format!("{}", style(value).cyan())
                    } else {
                        value.to_string()
                    }
                }
                Type::Accessor => {
                    let r#type = if let Some(subtype) = &self.subtype {
                        format!("{:?}", subtype)
                    } else {
                        "Object".to_string()
                    };
                    let mut value = format!("[{}]", r#type);
                    if cfg!(feature = "color") {
                        value = format!("{}", style(value).red())
                    }
                    value
                }
            }
        } else {
            let mut disp = format!("[{} {}]", &self.r#type, &self.name);
            if cfg!(feature = "color") {
                disp = format!("{}", style(disp).cyan());
            }
            disp
        }
    }
}

impl fmt::Display for PropertyPreview {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = &self.get_value();
        write!(f, "{}: {}", &self.name, value)
    }
}
