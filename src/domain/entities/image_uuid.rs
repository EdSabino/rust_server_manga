use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ImageUuid {
    pub uuid: String
}

impl ImageUuid {
    pub fn new(uuid: String) -> Self {
        ImageUuid {
            uuid: uuid
        }
    }
}