//! # SimConnect SDK
//! SimConnect SDK in Rust.
//!
//! ## Usage

//! ```toml
//! [dependencies]
//! simconnect-sdk = { version = "0.1", features = ["derive"] }
//! ```
//!
//! ```no_run
//! use simconnect_sdk::{Notification, SimConnect, SimConnectObject};
//!
//! /// A data structure that will be used to receive data from SimConnect.
//! #[derive(Debug, Clone, SimConnectObject)]
//! #[simconnect(period = "second")]
//! struct GpsData {
//!     #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
//!     lat: f64,
//!     #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
//!     lon: f64,
//!     #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
//!     alt: f64,
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = SimConnect::new("Simple Program");
//!
//!     match client {
//!         Ok(mut client) => loop {
//!             let notification = client.get_next_dispatch()?;
//!
//!             match notification {
//!                 Some(Notification::Open) => {
//!                     println!("Open");
//!
//!                     // After the connection is successfully open, we register the struct
//!                     client.register_object::<GpsData>()?;
//!                 }
//!                 Some(Notification::Object(data)) => {
//!                     if let Ok(gps_data) = GpsData::try_from(&data) {
//!                         println!("{gps_data:?}");
//!                     }
//!                 }
//!                 _ => (),
//!             }
//!
//!             // sleep for about a frame to reduce CPU usage
//!             std::thread::sleep(std::time::Duration::from_millis(16));
//!         },
//!         Err(e) => {
//!             println!("Error: {e:?}")
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! See [more examples](https://github.com/mihai-dinculescu/simconnect-sdk/tree/main/examples).

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
#[cfg_attr(docsrs, doc(cfg(feature = "simconnect-sdk-derive")))]
extern crate simconnect_sdk_derive;
#[cfg(feature = "simconnect-sdk-derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "simconnect-sdk-derive")))]
pub use simconnect_sdk_derive::*;
