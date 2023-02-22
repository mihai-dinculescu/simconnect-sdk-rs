use std::{collections::HashMap, ffi::c_void};

use tracing::{error, span, trace, warn, Level};

use crate::{
    as_c_string, bindings, helpers::fixed_c_str_to_string, ok_if_fail, success, Airport,
    ClientEvent, Notification, Object, SimConnectError, SystemEvent, Waypoint, CLIENT_EVENT_START,
    NDB, VOR,
};

/// SimConnect SDK Client.
///
/// # Example
///
/// ```rust,no_run
/// use simconnect_sdk::{Notification, SimConnect, SimConnectObject};
///
/// /// A data structure that will be used to receive data from SimConnect.
/// /// See the documentation of `SimConnectObject` for more information on the arguments of the `simconnect` attribute.
/// #[derive(Debug, Clone, SimConnectObject)]
/// #[simconnect(period = "second")]
/// #[allow(dead_code)]
/// struct AirplaneData {
///     #[simconnect(name = "TITLE")]
///     title: String,
///     #[simconnect(name = "CATEGORY")]
///     category: String,
///     #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
///     lat: f64,
///     #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
///     lon: f64,
///     #[simconnect(name = "PLANE ALTITUDE", unit = "feet")]
///     alt: f64,
///     #[simconnect(name = "SIM ON GROUND")]
///     sim_on_ground: bool,
/// }
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = SimConnect::new("Receiving data example");
///
///     match client {
///         Ok(mut client) => {
///             let mut notifications_received = 0;
///
///             loop {
///                 let notification = client.get_next_dispatch()?;
///
///                 match notification {
///                     Some(Notification::Open) => {
///                         println!("Connection opened.");
///
///                         // After the connection is successfully open, we register the struct
///                         client.register_object::<AirplaneData>()?;
///                     }
///                     Some(Notification::Object(data)) => {
///                         if let Ok(airplane_data) = AirplaneData::try_from(&data) {
///                             println!("{airplane_data:?}");
///
///                             notifications_received += 1;
///
///                             // After we have received 10 notifications, we unregister the struct
///                             if notifications_received > 10 {
///                                 client.unregister_object::<AirplaneData>()?;
///                                 println!("Subscription stopped.");
///                                 break;
///                             }
///                         }
///                     }
///                     _ => (),
///                 }
///
///                 // sleep for about a frame to reduce CPU usage
///                 std::thread::sleep(std::time::Duration::from_millis(16));
///             }
///         }
///         Err(e) => {
///             println!("Error: {e:?}")
///         }
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct SimConnect {
    pub(super) handle: std::ptr::NonNull<c_void>,
    pub(super) next_request_id: u32,
    pub(super) registered_objects: HashMap<String, RegisteredObject>,
}

/// A struct that represents a registered object.
#[derive(Debug)]
pub(super) struct RegisteredObject {
    pub id: u32,
    pub transient: bool,
}

impl RegisteredObject {
    pub(super) fn new(id: u32, transient: bool) -> Self {
        Self { id, transient }
    }
}

impl SimConnect {
    /// Create a new SimConnect SDK client.
    #[tracing::instrument(name = "SimConnect::new", level = "debug")]
    pub fn new(name: &str) -> Result<Self, SimConnectError> {
        let mut handle = std::ptr::null_mut();

        success!(unsafe {
            bindings::SimConnect_Open(
                &mut handle,
                as_c_string!(name),
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                0,
            )
        })?;

        Ok(Self {
            handle: std::ptr::NonNull::new(handle).ok_or_else(|| {
                SimConnectError::UnexpectedError(
                    "SimConnect_Open returned null pointer on success".to_string(),
                )
            })?,
            next_request_id: 0,
            registered_objects: HashMap::new(),
        })
    }

    /// Receive the next SimConnect message.
    ///
    /// # Remarks
    /// This is a non-blocking function. If there are no messages to receive, it will return None immediately.
    /// When called in a loop, it is recommended to use a short sleep time.
    pub fn get_next_dispatch(&mut self) -> Result<Option<Notification>, SimConnectError> {
        let mut data_buf: *mut bindings::SIMCONNECT_RECV = std::ptr::null_mut();
        let mut size_buf: bindings::DWORD = 32;
        let size_buf_pointer: *mut bindings::DWORD = &mut size_buf;

        unsafe {
            ok_if_fail!(
                bindings::SimConnect_GetNextDispatch(
                    self.handle.as_ptr(),
                    &mut data_buf,
                    size_buf_pointer
                ),
                None
            );
        };

        let recv_id = unsafe { (*data_buf).dwID as i32 };

        if recv_id == bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL {
            Ok(None)
        } else {
            let span = span!(Level::TRACE, "SimConnect::get_next_dispatch");
            let _enter = span.enter();

            match recv_id {
                bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => {
                    trace!("Received SIMCONNECT_RECV_OPEN");
                    Ok(Some(Notification::Open))
                }
                bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_QUIT => {
                    trace!("Received SIMCONNECT_RECV_QUIT");
                    Ok(Some(Notification::Quit))
                }
                bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => {
                    trace!("Received SIMCONNECT_RECV_EVENT");
                    let event: &bindings::SIMCONNECT_RECV_EVENT =
                        unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_EVENT) };

                    if event.uEventID >= CLIENT_EVENT_START {
                        let event = ClientEvent::try_from(event.uEventID)
                            .map_err(|_| SimConnectError::UnimplementedEventType(event.uEventID))?;

                        Ok(Some(Notification::ClientEvent(event)))
                    } else {
                        let event = SystemEvent::try_from(event)?;

                        Ok(Some(Notification::SystemEvent(event)))
                    }
                }
                bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_FILENAME => {
                    trace!("Received SIMCONNECT_RECV_EVENT_FILENAME");
                    let event: &bindings::SIMCONNECT_RECV_EVENT_FILENAME =
                        unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_EVENT_FILENAME) };

                    let event = SystemEvent::try_from(event)?;
                    Ok(Some(Notification::SystemEvent(event)))
                }
                bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_FRAME => {
                    trace!("Received SIMCONNECT_RECV_EVENT_FRAME");
                    let event: &bindings::SIMCONNECT_RECV_EVENT_FRAME =
                        unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_EVENT_FRAME) };

                    let event = SystemEvent::try_from(event)?;
                    Ok(Some(Notification::SystemEvent(event)))
                }
                bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA => {
                    trace!("Received SIMCONNECT_RECV_SIMOBJECT_DATA");

                    let event: &bindings::SIMCONNECT_RECV_SIMOBJECT_DATA =
                        unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_SIMOBJECT_DATA) };

                    let type_name = self.get_type_name_by_request_id(event.dwDefineID);

                    match type_name {
                        Some(type_name) => {
                            let data = Object {
                                type_name,
                                data_addr: std::ptr::addr_of!(event.dwData),
                            };

                            Ok(Some(Notification::Object(data)))
                        }
                        _ => Ok(None),
                    }
                }
                bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_AIRPORT_LIST => {
                    trace!("Received SIMCONNECT_RECV_AIRPORT_LIST");

                    let event: &bindings::SIMCONNECT_RECV_AIRPORT_LIST =
                        unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_AIRPORT_LIST) };

                    self.unregister_potential_transient_request(
                        event._base.dwEntryNumber,
                        event._base.dwOutOf,
                        event._base.dwRequestID,
                    );

                    let data = (0..event._base.dwArraySize as usize)
                        .map(|i| {
                            // `rgData` is defined as a 1-element array, but it is actually a variable-length array.
                            let record = unsafe { event.rgData.get_unchecked(i) };

                            Airport {
                                icao: fixed_c_str_to_string(&record.Icao),
                                lat: record.Latitude,
                                lon: record.Longitude,
                                alt: record.Altitude,
                            }
                        })
                        .collect::<Vec<_>>();

                    Ok(Some(Notification::AirportList(data)))
                }
                bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_WAYPOINT_LIST => {
                    trace!("Received SIMCONNECT_RECV_WAYPOINT_LIST");

                    let event: &bindings::SIMCONNECT_RECV_WAYPOINT_LIST =
                        unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_WAYPOINT_LIST) };

                    self.unregister_potential_transient_request(
                        event._base.dwEntryNumber,
                        event._base.dwOutOf,
                        event._base.dwRequestID,
                    );

                    let data = (0..event._base.dwArraySize as usize)
                        .map(|i| {
                            // `rgData` is defined as a 1-element array, but it is actually a variable-length array.
                            let record = unsafe { event.rgData.get_unchecked(i) };

                            Waypoint {
                                icao: fixed_c_str_to_string(&record._base.Icao),
                                lat: record._base.Latitude,
                                lon: record._base.Longitude,
                                alt: record._base.Altitude,
                                mag_var: record.fMagVar,
                            }
                        })
                        .collect::<Vec<_>>();

                    Ok(Some(Notification::WaypointList(data)))
                }
                bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NDB_LIST => {
                    trace!("Received SIMCONNECT_RECV_NDB_LIST");

                    let event: &bindings::SIMCONNECT_RECV_NDB_LIST =
                        unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_NDB_LIST) };

                    self.unregister_potential_transient_request(
                        event._base.dwEntryNumber,
                        event._base.dwOutOf,
                        event._base.dwRequestID,
                    );

                    let data = (0..event._base.dwArraySize as usize)
                        .map(|i| {
                            // `rgData` is defined as a 1-element array, but it is actually a variable-length array.
                            let record = unsafe { event.rgData.get_unchecked(i) };

                            NDB {
                                icao: fixed_c_str_to_string(&record._base._base.Icao),
                                lat: record._base._base.Latitude,
                                lon: record._base._base.Longitude,
                                alt: record._base._base.Altitude,
                                mag_var: record._base.fMagVar,
                                frequency: record.fFrequency,
                            }
                        })
                        .collect::<Vec<_>>();

                    Ok(Some(Notification::NdbList(data)))
                }
                bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_VOR_LIST => {
                    trace!("Received SIMCONNECT_RECV_VOR_LIST");

                    let event: &bindings::SIMCONNECT_RECV_VOR_LIST =
                        unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_VOR_LIST) };

                    self.unregister_potential_transient_request(
                        event._base.dwEntryNumber,
                        event._base.dwOutOf,
                        event._base.dwRequestID,
                    );

                    let data = (0..event._base.dwArraySize as usize)
                        .map(|i| {
                            // `rgData` is defined as a 1-element array, but it is actually a variable-length array.
                            let record = unsafe { event.rgData.get_unchecked(i) };

                            let has_nav_signal = record.Flags
                                & bindings::SIMCONNECT_RECV_ID_VOR_LIST_HAS_NAV_SIGNAL
                                == bindings::SIMCONNECT_RECV_ID_VOR_LIST_HAS_NAV_SIGNAL;
                            let has_localizer = record.Flags
                                & bindings::SIMCONNECT_RECV_ID_VOR_LIST_HAS_LOCALIZER
                                == bindings::SIMCONNECT_RECV_ID_VOR_LIST_HAS_LOCALIZER;
                            let has_glide_slope = record.Flags
                                & bindings::SIMCONNECT_RECV_ID_VOR_LIST_HAS_GLIDE_SLOPE
                                == bindings::SIMCONNECT_RECV_ID_VOR_LIST_HAS_GLIDE_SLOPE;
                            let has_dme = record.Flags
                                & bindings::SIMCONNECT_RECV_ID_VOR_LIST_HAS_DME
                                == bindings::SIMCONNECT_RECV_ID_VOR_LIST_HAS_DME;

                            VOR {
                                icao: fixed_c_str_to_string(&record._base._base._base.Icao),
                                lat: record._base._base._base.Latitude,
                                lon: record._base._base._base.Longitude,
                                alt: record._base._base._base.Altitude,
                                mag_var: record._base._base.fMagVar,
                                has_nav_signal,
                                has_localizer,
                                has_glide_slope,
                                has_dme,
                                localizer: if has_localizer {
                                    Some(record.fLocalizer)
                                } else {
                                    None
                                },
                                glide_lat: if has_nav_signal {
                                    Some(record.GlideLat)
                                } else {
                                    None
                                },
                                glide_lon: if has_nav_signal {
                                    Some(record.GlideLon)
                                } else {
                                    None
                                },
                                glide_alt: if has_nav_signal {
                                    Some(record.GlideAlt)
                                } else {
                                    None
                                },
                                glide_slope_angle: if has_glide_slope {
                                    Some(record.fGlideSlopeAngle)
                                } else {
                                    None
                                },
                                frequency: if has_dme {
                                    Some(record._base.fFrequency)
                                } else {
                                    None
                                },
                            }
                        })
                        .collect::<Vec<_>>();

                    Ok(Some(Notification::VorList(data)))
                }
                bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EXCEPTION => {
                    let event: &bindings::SIMCONNECT_RECV_EXCEPTION =
                        unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_EXCEPTION) };

                    warn!("Received {:?}", event);

                    Err(SimConnectError::SimConnectException(event.dwException))
                }
                bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL => Ok(None),
                id => {
                    error!("Received unhandled notification ID: {}", id);
                    Err(SimConnectError::UnimplementedMessageType(id))
                }
            }
        }
    }

    /// Register a Request ID in the internal state so that the user doesn't have to manually manage Request IDs.
    #[tracing::instrument(name = "SimConnect::new_request_id", level = "trace", skip(self))]
    pub(super) fn new_request_id(
        &mut self,
        type_name: String,
        transient: bool,
    ) -> Result<u32, SimConnectError> {
        if self.registered_objects.contains_key(&type_name) {
            return Err(SimConnectError::ObjectAlreadyRegistered(type_name));
        }

        let mut request_id = self.next_request_id;
        self.next_request_id += 1;

        // when `next_request_id` overflows some ids might still be in use
        // so we need to find the next available one
        while self
            .registered_objects
            .values()
            .any(|obj| obj.id == request_id)
        {
            request_id = self.next_request_id;
            self.next_request_id += 1;
        }

        self.registered_objects
            .insert(type_name, RegisteredObject::new(request_id, transient));

        Ok(request_id)
    }

    /// Unregister a Request ID in the internal state so that the user doesn't have to manually manage Request IDs.
    #[tracing::instrument(
        name = "SimConnect::unregister_request_id_by_type_name",
        level = "trace",
        skip(self)
    )]
    pub(super) fn unregister_request_id_by_type_name(&mut self, type_name: &str) -> Option<u32> {
        self.registered_objects.remove(type_name).map(|obj| obj.id)
    }

    /// Get the Type Name of a Request ID.
    #[tracing::instrument(
        name = "SimConnect::get_type_name_by_request_id",
        level = "trace",
        skip(self)
    )]
    pub(super) fn get_type_name_by_request_id(&self, request_id: u32) -> Option<String> {
        self.registered_objects
            .iter()
            .find(|(_, v)| v.id == request_id)
            .map(|(k, _)| k.clone())
    }

    /// Get the Type Name of a Request ID.
    #[tracing::instrument(name = "SimConnect::is_transient_request", level = "trace", skip(self))]
    pub(super) fn is_transient_request(&self, request_id: u32) -> Option<bool> {
        self.registered_objects
            .iter()
            .find(|(_, v)| v.id == request_id)
            .map(|(_, v)| v.transient)
    }

    /// Checks if the request is
    /// 1) the last entry in the list and
    /// 2) transient
    /// and if yes, it gets unregistered.
    #[tracing::instrument(
        name = "SimConnect::unregister_potential_transient_request",
        level = "trace",
        fields(type_name, transient),
        skip(self)
    )]
    pub(super) fn unregister_potential_transient_request(
        &mut self,
        entry_number: u32,
        out_of: u32,
        request_id: u32,
    ) {
        if entry_number + 1 >= out_of {
            // This is the last entry, so we can clear the request if it's transient.
            let transient = self.is_transient_request(request_id);
            tracing::Span::current().record("transient", transient);
            if self.is_transient_request(request_id) == Some(true) {
                let type_name = self.get_type_name_by_request_id(request_id);

                if let Some(ref type_name) = type_name {
                    tracing::Span::current().record("type_name", type_name);

                    trace!("Clearing");
                    self.unregister_request_id_by_type_name(type_name);
                }
            }
        }
    }
}

impl Drop for SimConnect {
    #[tracing::instrument(name = "SimConnect::drop", level = "debug", skip(self))]
    fn drop(&mut self) {
        let _ = unsafe { bindings::SimConnect_Close(self.handle.as_ptr()) };
    }
}
