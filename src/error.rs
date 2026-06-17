use serde::{ser::Serializer, Serialize};
use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid symbol request: {0}")]
    InvalidRequest(String),
    #[error("{system} is not supported on {platform}")]
    UnsupportedPlatform {
        system: &'static str,
        platform: &'static str,
    },
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[cfg(windows)]
    #[error(transparent)]
    Windows(#[from] windows_core::Error),
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
