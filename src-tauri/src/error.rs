use core::fmt;

use crate::modules::task_assignator::SuggestionResult;

impl fmt::Display for SuggestionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self) // Customize the formatting as needed
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Failed to parse date: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Assignator error: {0}")]
    Assignator(SuggestionResult),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
