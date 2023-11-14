use serde::{Deserialize, Serialize};

const DETERMINISTIC: &str = "deterministic";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Language {
    pub policy: String,
    pub code: String,
}

impl Language {
    pub fn new(code: &str) -> Language {
        Self {
            policy: DETERMINISTIC.into(),
            code: code.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<U> {
    pub status_code: u16,
    pub data: U,
}
