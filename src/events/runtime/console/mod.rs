use crate::events::runtime::remote_object::RemoteObject;

use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsoleEvent {
    #[serde(rename = "type")]
    pub log_type: String,
    #[serde(rename = "args")]
    pub messages: Vec<RemoteObject>,
}

impl fmt::Display for ConsoleEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let last_index = self.messages.len() - 1;
        for (index, message) in self.messages.iter().enumerate() {
            write!(f, "{}", message)?;
            if index < last_index {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}
