use thiserror::Error;

/// SimConnect SDK error.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SimConnectError {
    /// SimConnect error.
    #[error("SimConnect error: {0}")]
    SimConnectError(i32),
    /// SimConnect error.
    #[error("SimConnect exception: {0}")]
    SimConnectException(u32),
    /// An unimplemented event type has been received by the SDK.
    #[error("Unimplemented event in the SDK: {0}")]
    UnimplementedEventType(u32),
    /// An unimplemented message type has been received by the SDK.
    #[error("Unimplemented notification in the SDK: {0}")]
    UnimplementedMessageType(i32),
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
