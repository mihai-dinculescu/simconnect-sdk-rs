//! # SimConnect SDK
//! SimConnect SDK in Rust.
//!
//! See [examples](https://github.com/mihai-dinculescu/simconnect-sdk/tree/main/examples/src).

mod bindings;
mod domain;
mod errors;
mod helpers;
mod macros;
mod simconnect;
mod simconnect_object_ext;

pub(crate) use macros::{as_c_string, ok_if_fail, success};

pub use domain::*;
pub use errors::SimConnectError;
pub use simconnect::SimConnect;
pub use simconnect_object_ext::SimConnectObjectExt;

#[cfg(feature = "simconnect-sdk-derive")]
extern crate simconnect_sdk_derive;
#[cfg(feature = "simconnect-sdk-derive")]
pub use simconnect_sdk_derive::*;
