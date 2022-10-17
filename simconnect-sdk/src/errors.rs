use thiserror::Error;

/// SimConnect SDK error.
#[derive(Error, Debug)]
pub enum SimConnectError {
    /// SimConnect error.
    #[error("SimConnect error: {0}")]
    SimConnectError(i32),
    #[error("SimConnect unrecognized: {0}")]
    /// SimConnect unrecognized error. Occurs when an unimplemented event is received by the SDK.
    SimConnectUnrecognizedEvent(u32),
    /// Object already registered with the client instance.
    #[error("Object `{0}` has already been registered")]
    ObjectAlreadyRegistered(String),
    /// Object already registered with the client instance.
    #[error("Object `{0}` has not been registered")]
    ObjectNotRegistered(String),
    /// Object mismatch.
    #[error("Tried to convert object of type {actual} to {expected}")]
    ObjectMismatch { actual: String, expected: String },
    /// Conversation error.
    #[error("Conversion error: {0}")]
    ConversionError(#[from] std::num::TryFromIntError),
    /// Unexpected error.
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}
