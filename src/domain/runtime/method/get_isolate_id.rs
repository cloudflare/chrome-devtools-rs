use serde::{Deserialize, Serialize};

use crate::domain::runtime::method::r#type::{Id, IsolateId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Return {
    id: Id,
    isolate_id: IsolateId,
}
