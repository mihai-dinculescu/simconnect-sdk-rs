//! # SimConnect SDK
//! SimConnect SDK in Rust.
//!
//! ## Usage
//! ```toml
//! [dependencies]
//! simconnect-sdk = { version = "0.2", features = ["derive"] }
//! ```
//!
//! ```rust,no_run
//! use simconnect_sdk::{Notification, SimConnect, SimConnectObject};
//!
//! /// A data structure that will be used to receive data from SimConnect.
//! /// See the documentation of `SimConnectObject` for more information on the arguments of the `simconnect` attribute.
//! #[derive(Debug, Clone, SimConnectObject)]
//! #[simconnect(period = "second")]
//! #[allow(dead_code)]
//! struct AirplaneData {
//!     #[simconnect(name = "TITLE")]
//!     title: String,
//!     #[simconnect(name = "CATEGORY")]
//!     category: String,
//!     #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
//!     lat: f64,
//!     #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
//!     lon: f64,
//!     #[simconnect(name = "PLANE ALTITUDE", unit = "feet")]
//!     alt: f64,
//!     #[simconnect(name = "SIM ON GROUND")]
//!     sim_on_ground: bool,
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = SimConnect::new("Receiving data example");
//!
//!     match client {
//!         Ok(mut client) => {
//!             let mut notifications_received = 0;
//!
//!             loop {
//!                 let notification = client.get_next_dispatch()?;
//!
//!                 match notification {
//!                     Some(Notification::Open) => {
//!                         println!("Connection opened.");
//!
//!                         // After the connection is successfully open, we register the struct
//!                         client.register_object::<AirplaneData>()?;
//!                     }
//!                     Some(Notification::Object(data)) => {
//!                         if let Ok(airplane_data) = AirplaneData::try_from(&data) {
//!                             println!("{airplane_data:?}");
//!
//!                             notifications_received += 1;
//!
//!                             // After we have received 10 notifications, we unregister the struct
//!                             if notifications_received > 10 {
//!                                 client.unregister_object::<AirplaneData>()?;
//!                                 println!("Subscription stopped.");
//!                                 break;
//!                             }
//!                         }
//!                     }
//!                     _ => (),
//!                 }
//!
//!                 // sleep for about a frame to reduce CPU usage
//!                 std::thread::sleep(std::time::Duration::from_millis(16));
//!             }
//!         }
//!         Err(e) => {
//!             println!("Error: {e:?}")
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! See [more examples](https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/main/examples).

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
pub use helpers::fixed_c_str_to_string;
pub use simconnect::SimConnect;
pub use simconnect_object_ext::SimConnectObjectExt;

#[cfg(feature = "simconnect-sdk-derive")]
extern crate simconnect_sdk_derive;
#[cfg(feature = "simconnect-sdk-derive")]
pub use simconnect_sdk_derive::*;
