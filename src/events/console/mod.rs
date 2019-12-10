mod primitive;

<<<<<<< HEAD
pub use primitive::JsPrimitive;

use std::fmt;
=======
use primitive::JsPrimitive;
>>>>>>> Initial commit

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsoleEvent {
    #[serde(rename = "type")]
    pub log_type: String,
    #[serde(rename = "args")]
    pub messages: Vec<JsPrimitive>,
}
<<<<<<< HEAD

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
=======
>>>>>>> Initial commit
