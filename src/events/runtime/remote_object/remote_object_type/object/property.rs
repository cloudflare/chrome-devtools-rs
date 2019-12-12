use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectProperty {
    pub name: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub value: Option<String>,
}

impl fmt::Display for ObjectProperty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.object_type.as_str() {
            "string" => match &self.value {
                Some(value) => write!(f, "\"{}\": \"{}\"", &self.name, value),
                None => write!(f, "\"{}\"", &self.name),
            },
            _ => write!(f, "[{} {}]", &self.object_type, &self.name),
        }
    }
}
