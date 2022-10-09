use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimConnectError {
    #[error("SimConnect error: {0}")]
    SimConnectError(i32),
    #[error("SimConnect unrecognized: {0}")]
    SimConnectUnrecognizedEvent(u32),
    #[error("Object `{0}` has already been registered")]
    ObjectAlreadyRegistered(String),
    #[error("Conversion error: {0}")]
    ConversionError(#[from] std::num::TryFromIntError),
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}
