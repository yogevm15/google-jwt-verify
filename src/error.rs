use crate::algorithm::Algorithm;
use base64::DecodeError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Invalid token.")]
    InvalidToken,
    #[error("Failed to retrieve key.")]
    RetrieveKeyFailure,
    #[error("Unsupported algorithm: `{0:?}`.")]
    UnsupportedAlgorithm(Algorithm),
    #[error("Token expired.")]
    Expired,
}

impl From<DecodeError> for Error {
    fn from(_: DecodeError) -> Self {
        Error::InvalidToken
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::InvalidToken
    }
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(_: openssl::error::ErrorStack) -> Self {
        Error::InvalidToken
    }
}
