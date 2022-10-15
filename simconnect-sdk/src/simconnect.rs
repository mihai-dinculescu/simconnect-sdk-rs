use std::ffi::c_void;

use crate::{
    as_c_string, bindings, helpers::fixed_c_str_to_string, ok_if_fail, success, Airport, Condition,
    DataType, Event, FacilityType, Notification, NotificationGroup, Object, Period,
    SimConnectError, SimConnectObjectExt, Waypoint, NDB, VOR,
};

/// SimConnect SDK Client.
///
/// # Example
///
/// ```no_run
/// use simconnect_sdk::{Notification, SimConnect, SimConnectObject};
///
/// /// A data structure that will be used to receive data from SimConnect.
/// #[derive(Debug, Clone, SimConnectObject)]
/// #[simconnect(period = "second")]
/// struct GpsData {
///     #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
///     lat: f64,
///     #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
///     lon: f64,
///     #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
///     alt: f64,
/// }
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = SimConnect::new("Simple Program");
///
///     match client {
///         Ok(mut client) => loop {
///             let notification = client.get_next_dispatch()?;
///
///             match notification {
///                 Some(Notification::Open) => {
///                     println!("Open");
///
///                     // After the connection is successfully open, we register the struct
///                     client.register_object::<GpsData>()?;
///                 }
///                 Some(Notification::Object(data)) => {
///                     if let Ok(gps_data) = GpsData::try_from(&data) {
///                         println!("{gps_data:?}");
///                     }
///                 }
///                 _ => (),
///             }
///
///             // sleep for about a frame to reduce CPU usage
///             std::thread::sleep(std::time::Duration::from_millis(16));
///         },
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
    handle: std::ptr::NonNull<c_void>,
    registered_objects: Vec<String>,
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
            registered_objects: Vec::new(),
        })
    }

    // Register an object with SimConnect by assigning it an unique interval `request_id` and then calling the [`crate::SimConnectObjectExt::register`] method on the struct.
    #[tracing::instrument(name = "SimConnect::register_object")]
    pub fn register_object<T: SimConnectObjectExt>(&mut self) -> Result<u32, SimConnectError> {
        let type_name: String = std::any::type_name::<T>().into();

        let id = self.register_request_id(type_name)?;

        T::register(self, id)?;

        Ok(id)
    }

    /// Associates a client defined event with a Microsoft Flight Simulator event name.
    ///
    /// WIP
    #[tracing::instrument(name = "SimConnect::register_event")]
    pub fn register_event(
        &self,
        event: Event,
        notification_group: NotificationGroup,
    ) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_MapClientEventToSimEvent(
                self.handle.as_ptr(),
                event as u32,
                event.into_c_char(),
            )
        });

        success!(unsafe {
            bindings::SimConnect_AddClientEventToNotificationGroup(
                self.handle.as_ptr(),
                notification_group as u32,
                event as u32,
                0,
            )
        });

        success!(unsafe {
            bindings::SimConnect_SetNotificationGroupPriority(
                self.handle.as_ptr(),
                notification_group as u32,
                1,
            )
        });

        Ok(())
    }

    /// Add a Microsoft Flight Simulator simulation variable name to a client defined object definition.
    ///
    /// # Remarks
    /// The [`crate::SimConnectObject`] macro will automatically call this method for you.
    #[tracing::instrument(name = "SimConnect::add_to_data_definition")]
    pub fn add_to_data_definition(
        &self,
        request_id: u32,
        datum_name: &str,
        units_name: &str,
        data_type: DataType,
    ) -> Result<(), SimConnectError> {
        let c_type = match data_type {
            DataType::Float64 => bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
            DataType::Bool => bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_INT32,
        };

        unsafe {
            success!(bindings::SimConnect_AddToDataDefinition(
                self.handle.as_ptr(),
                request_id,
                as_c_string!(datum_name),
                as_c_string!(units_name),
                c_type,
                0.0,
                u32::MAX,
            ));
        }

        Ok(())
    }

    /// Request when the SimConnect client is to receive data values for a specific object.
    ///
    /// # Remarks
    /// The [`crate::SimConnectObject`] macro will automatically call this method for you.
    ///
    /// It is possible to change the period of a request, by re-sending the [`crate::SimConnect::request_data_on_sim_object`] call with the same `request_id` parameters, but with a new `period`.
    /// The one exception to this is the new period cannot be [`crate::Period::Once`], in this case a request with a new `request_id` should be sent.
    #[tracing::instrument(name = "SimConnect::request_data_on_sim_object")]
    pub fn request_data_on_sim_object(
        &self,
        request_id: u32,
        period: Period,
        condition: Condition,
        interval: u32,
    ) -> Result<(), SimConnectError> {
        unsafe {
            success!(bindings::SimConnect_RequestDataOnSimObject(
                self.handle.as_ptr(),
                request_id,
                request_id,
                request_id,
                period.into(),
                condition.into(),
                0,
                interval,
                0,
            ));
        }

        Ok(())
    }

    /// Request notifications when a facility of a certain type is added to the facilities cache.
    ///
    /// When this function is first called, a full list from the cache will be sent, thereafter just the additions will be transmitted.
    /// No notification is given when a facility is removed from the cache.
    /// To terminate these notifications use the [`crate::SimConnect::unsubscribe_to_facilities`] function.
    ///
    /// # Remarks
    /// The simulation keeps a facilities cache of all the airports, waypoints, NDB and VOR stations within a certain radius of the user aircraft.
    /// This radius varies depending on where the aircraft is in the world, but is at least large enough to encompass the whole of the reality bubble for airports and waypoints, and can be over 200 miles for VOR and NDB stations.
    /// As the user aircraft moves facilities will be added to, and removed from, the cache. However, in the interests pf performance, hysteresis is built into the system.
    #[tracing::instrument(name = "SimConnect::subscribe_to_facilities")]
    pub fn subscribe_to_facilities(
        &mut self,
        facility_type: FacilityType,
    ) -> Result<(), SimConnectError> {
        let type_name = facility_type.to_type_name();
        let request_id = self.register_request_id(type_name)?;

        unsafe {
            success!(bindings::SimConnect_SubscribeToFacilities(
                self.handle.as_ptr(),
                facility_type.into(),
                request_id,
            ));
        }

        Ok(())
    }

    /// Request a list of all the facilities of a given type currently held in the facilities cache.
    ///
    /// # Remarks
    /// The simulation keeps a facilities cache of all the airports, waypoints, NDB and VOR stations within a certain radius of the user aircraft.
    /// This radius varies depending on where the aircraft is in the world, but is at least large enough to encompass the whole of the reality bubble for airports and waypoints, and can be over 200 miles for VOR and NDB stations.
    /// As the user aircraft moves facilities will be added to, and removed from, the cache. However, in the interests pf performance, hysteresis is built into the system.
    #[tracing::instrument(name = "SimConnect::request_facilities_list")]
    pub fn request_facilities_list(
        &mut self,
        facility_type: FacilityType,
    ) -> Result<(), SimConnectError> {
        let type_name = facility_type.to_type_name();
        let request_id = self.register_request_id(type_name)?;

        unsafe {
            success!(bindings::SimConnect_RequestFacilitiesList(
                self.handle.as_ptr(),
                facility_type.into(),
                request_id,
            ));
        }

        Ok(())
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

        let result = match unsafe { (*data_buf).dwID as i32 } {
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => Some(Notification::Open),
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_QUIT => Some(Notification::Quit),
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => {
                let event = unsafe { *(data_buf as *const bindings::SIMCONNECT_RECV_EVENT) };
                let event = Event::try_from(event.uEventID)
                    .map_err(|_| SimConnectError::SimConnectUnrecognizedEvent(event.uEventID))?;
                Some(Notification::Event(event))
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA => {
                let event: &bindings::SIMCONNECT_RECV_SIMOBJECT_DATA = unsafe {
                    std::mem::transmute_copy(
                        &(data_buf as *const bindings::SIMCONNECT_RECV_SIMOBJECT_DATA),
                    )
                };

                let object_type = self.registered_objects.get(event.dwDefineID as usize);

                match object_type {
                    Some(object_type) => {
                        let data = Object {
                            type_name: object_type.clone(),
                            data_addr: std::ptr::addr_of!(event.dwData),
                        };

                        Some(Notification::Object(data))
                    }
                    _ => None,
                }
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_AIRPORT_LIST => {
                let event: &bindings::SIMCONNECT_RECV_AIRPORT_LIST = unsafe {
                    std::mem::transmute_copy(
                        &(data_buf as *const bindings::SIMCONNECT_RECV_AIRPORT_LIST),
                    )
                };

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
                let event: &bindings::SIMCONNECT_RECV_WAYPOINT_LIST = unsafe {
                    std::mem::transmute_copy(
                        &(data_buf as *const bindings::SIMCONNECT_RECV_WAYPOINT_LIST),
                    )
                };

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
                let event: &bindings::SIMCONNECT_RECV_NDB_LIST = unsafe {
                    std::mem::transmute_copy(
                        &(data_buf as *const bindings::SIMCONNECT_RECV_NDB_LIST),
                    )
                };

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
                let event: &bindings::SIMCONNECT_RECV_VOR_LIST = unsafe {
                    std::mem::transmute_copy(
                        &(data_buf as *const bindings::SIMCONNECT_RECV_VOR_LIST),
                    )
                };

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
                let event = unsafe { *(data_buf as *const bindings::SIMCONNECT_RECV_EXCEPTION) };
                Some(Notification::Exception(event.dwException))
            }
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL => None,
            _ => panic!("Got unrecognized notification: {}", unsafe {
                (*data_buf).dwID as i32
            }),
        };

        Ok(result)
    }

    /// Register a request id in the internal state so that the user doesn't have to manually manage requests ids.
    #[tracing::instrument(name = "SimConnect::register_request_id")]
    fn register_request_id(&mut self, type_name: String) -> Result<u32, SimConnectError> {
        if self.registered_objects.contains(&type_name) {
            return Err(SimConnectError::ObjectAlreadyRegistered(type_name));
        }

        self.registered_objects.push(type_name.clone());

        // using the index for now because we don't unregister objects, yet
        let id = self
            .registered_objects
            .iter()
            .position(|p| p == &type_name)
            .ok_or_else(|| {
                SimConnectError::UnexpectedError("failed to find registered event".to_string())
            })?;
        let id = u32::try_from(id)?;

        Ok(id)
    }
}

impl Drop for SimConnect {
    fn drop(&mut self) {
        let _ = unsafe { bindings::SimConnect_Close(self.handle.as_ptr()) };
    }
}
