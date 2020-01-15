use std::fmt;

use crate::domain::runtime::r#type::remote_object::r#type::object::Preview;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub key: Preview,
    pub value: Preview,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}", &self.key, &self.value)
    }
}
