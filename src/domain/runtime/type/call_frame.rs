/// See [CallFrame](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#type-CallFrame)

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CallFrame {
    pub function_name: String,
    pub script_id: String,
    pub url: String,
    pub line_number: i32,
    pub column_number: i32,
}