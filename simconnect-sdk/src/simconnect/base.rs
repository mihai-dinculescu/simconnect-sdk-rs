use std::{collections::HashMap, ffi::c_void};

use crate::{
    as_c_string, bindings, helpers::fixed_c_str_to_string, ok_if_fail, success, Airport, Event,
    Notification, Object, SimConnectError, Waypoint, NDB, VOR,
};

/// SimConnect SDK Client.
///
/// # Example
///
/// ```rust,no_run
#[doc = include_str!("../../../examples/src/data.rs")]
/// ```
#[derive(Debug)]
pub struct SimConnect {
    pub(super) handle: std::ptr::NonNull<c_void>,
    pub(super) next_request_id: u32,
    pub(super) registered_objects: HashMap<String, u32>,
}

impl SimConnect {
    /// Create a new SimConnect SDK client.
    #[tracing::instrument(name = "SimConnect::new")]
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
        });

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
    pub fn get_next_dispatch(&self) -> Result<Option<Notification>, SimConnectError> {
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

        let result = match recv_id {
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => Some(Notification::Open),
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_QUIT => Some(Notification::Quit),
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => {
                let event: &bindings::SIMCONNECT_RECV_EVENT =
                    unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_EVENT) };

                let event = Event::try_from(event.uEventID)
                    .map_err(|_| SimConnectError::SimConnectUnrecognizedEvent(event.uEventID))?;

                Some(Notification::Event(event))
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA => {
                let event: &bindings::SIMCONNECT_RECV_SIMOBJECT_DATA =
                    unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_SIMOBJECT_DATA) };

                let type_name = self.get_type_name_by_request_id(event.dwDefineID);

                match type_name {
                    Some(type_name) => {
                        let data = Object {
                            type_name,
                            data_addr: std::ptr::addr_of!(event.dwData),
                        };

                        Some(Notification::Object(data))
                    }
                    _ => None,
                }
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_AIRPORT_LIST => {
                let event: &bindings::SIMCONNECT_RECV_AIRPORT_LIST =
                    unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_AIRPORT_LIST) };

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

                Some(Notification::AirportList(data))
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_WAYPOINT_LIST => {
                let event: &bindings::SIMCONNECT_RECV_WAYPOINT_LIST =
                    unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_WAYPOINT_LIST) };

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

                Some(Notification::WaypointList(data))
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NDB_LIST => {
                let event: &bindings::SIMCONNECT_RECV_NDB_LIST =
                    unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_NDB_LIST) };

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

                Some(Notification::NdbList(data))
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_VOR_LIST => {
                let event: &bindings::SIMCONNECT_RECV_VOR_LIST =
                    unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_VOR_LIST) };

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
                        let has_dme = record.Flags & bindings::SIMCONNECT_RECV_ID_VOR_LIST_HAS_DME
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

                Some(Notification::VorList(data))
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EXCEPTION => {
                let event: &bindings::SIMCONNECT_RECV_EXCEPTION =
                    unsafe { &*(data_buf as *const bindings::SIMCONNECT_RECV_EXCEPTION) };
                Some(Notification::Exception(event.dwException))
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL => None,
            id => panic!("Got unrecognized notification: {id}"),
        };

        Ok(result)
    }

    /// Register a Request ID in the internal state so that the user doesn't have to manually manage Request IDs.
    #[tracing::instrument(name = "SimConnect::new_request_id")]
    pub(super) fn new_request_id(&mut self, type_name: String) -> Result<u32, SimConnectError> {
        if self.registered_objects.contains_key(&type_name) {
            return Err(SimConnectError::ObjectAlreadyRegistered(type_name));
        }

        let mut request_id = self.next_request_id;
        self.next_request_id += 1;

        // when `next_request_id` overflows some ids might still be in use
        // so we need to find the next available one
        while self.registered_objects.values().any(|id| *id == request_id) {
            request_id = self.next_request_id;
            self.next_request_id += 1;
        }

        self.registered_objects.insert(type_name, request_id);

        Ok(request_id)
    }

    /// Unregister a Request ID in the internal state so that the user doesn't have to manually manage Request IDs.
    #[tracing::instrument(name = "SimConnect::unregister_request_id_by_type_name")]
    pub(super) fn unregister_request_id_by_type_name(&mut self, type_name: &str) -> Option<u32> {
        self.registered_objects.remove(type_name)
    }

    /// Get the Type Name of a Request ID.
    #[tracing::instrument(name = "SimConnect::get_type_name_by_request_id")]
    pub(super) fn get_type_name_by_request_id(&self, request_id: u32) -> Option<String> {
        self.registered_objects
            .iter()
            .find(|(_, v)| **v == request_id)
            .map(|(k, _)| k.clone())
    }
}

impl Drop for SimConnect {
    fn drop(&mut self) {
        let _ = unsafe { bindings::SimConnect_Close(self.handle.as_ptr()) };
    }
}
