use crate::domain::runtime::r#type::RemoteObject;

use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub r#type: String,
    pub args: Vec<RemoteObject>,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let last_index = self.args.len() - 1;
        for (index, arg) in self.args.iter().enumerate() {
            write!(f, "{}", arg)?;
            if index < last_index {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}
