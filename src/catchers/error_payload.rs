use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub struct ErrorPayload {
    pub result: bool,
    pub message: String
}

impl ErrorPayload {
    pub fn new(e: String) -> Self {
        ErrorPayload {
            result: false,
            message: e
        }
    }
}