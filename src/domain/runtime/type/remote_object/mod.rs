use std::fmt;

pub mod r#type;

use r#type::object::{ObjectPreview, Subtype};
use r#type::Type;

use serde::{Deserialize, Serialize};
use snailquote::unescape;

use console::style;

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
    fn parse_error(&self) -> String {
        let e = if let Some(subtype) = &self.subtype {
            format!("{}", subtype)
        } else {
            format!("{}", &self.r#type)
        };
        format!("{{unknown {}}}", e)
    }

    fn display_description(&self) -> String {
        if let Some(description) = &self.description {
            description.to_string()
        } else {
            self.parse_error()
        }
    }

    fn display_value(&self) -> String {
        if let Some(value) = &self.value {
            value.to_string()
        } else {
            self.parse_error()
        }
    }

    fn display_object(&self) -> String {
        if let Some(preview) = &self.preview {
            preview.to_string()
        } else if let Some(subtype) = &self.subtype {
            if subtype.to_string() == "null" {
                let mut null = "null".to_string();
                if cfg!(feature = "color") {
                    null = format!("{}", style(null).bold());
                }
                null
            } else {
                self.parse_error()
            }
        } else {
            self.parse_error()
        }
    }
}

impl fmt::Display for RemoteObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = match &self.r#type {
            Type::Undefined => {
                let mut disp = "undefined".to_string();
                if cfg!(feature = "color") {
                    disp = format!("{}", style(disp).dim())
                }
                disp
            }
            Type::Boolean => {
                let mut disp = self.display_value();
                if cfg!(feature = "color") {
                    disp = format!("{}", style(disp).yellow());
                }
                disp
            }
            Type::String => {
                if let Some(value) = &self.value {
                    let value = value.to_string();
                    let end = value.len() - 1;

                    if let Ok(s) = unescape(&value[1..end]) {
                        s
                    } else {
                        self.parse_error()
                    }
                } else {
                    self.parse_error()
                }
            }
            Type::Object => self.display_object(),
            Type::BigInt | Type::Number | Type::Symbol => {
                let mut disp = self.display_description();
                if cfg!(feature = "color") {
                    disp = format!("{}", style(disp).yellow());
                }
                disp
            }
            Type::Function => {
                let mut disp = self.display_description();
                if cfg!(feature = "color") {
                    disp = format!("{}", style(disp).cyan());
                }
                disp
            }
            Type::Accessor => self.display_object(),
        };
        write!(f, "{}", formatted)
    }
}
