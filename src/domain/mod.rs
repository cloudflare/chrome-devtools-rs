pub mod runtime;

pub use runtime::Runtime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Domain {
    Runtime(Runtime),
}
