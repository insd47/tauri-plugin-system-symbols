use serde::{ser::Serializer, Serialize};

use crate::models::SymbolFamily;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid symbol request: {0}")]
    InvalidRequest(String),
    #[error("{family} is not supported on this platform yet")]
    UnsupportedPlatform { family: SymbolFamily },
    #[error("{0}")]
    Symbol(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
