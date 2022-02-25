use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct ErrorData {
    code: i32,
    message: String,
    description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Hash, Error)]
pub struct Error {
    pub error: ErrorData,
    pub id: Option<i64>,
}

impl Error {
    pub fn code(&self) -> i32 {
        self.error.code
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. ", self.error.message)?;
        if let Some(extra) = &self.error.description {
            write!(f, "{}. ", extra)?;
        }
        write!(f, "code: {}", self.error.code)?;
        Ok(())
    }
}
