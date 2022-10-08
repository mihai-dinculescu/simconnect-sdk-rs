use std::ffi::c_void;

use crate::{
    as_c_string, bindings, helpers::fixed_c_str_to_string, ok_if_fail, success, AirportData,
    ConditionEnum, DataType, Event, Group, Notification, NotificationData, PeriodEnum,
    SimConnectError, SimConnectObject,
};

#[derive(Debug)]
pub struct SimConnect {
    pub handle: std::ptr::NonNull<c_void>,
    pub registered_objects: Vec<String>,
}

impl SimConnect {
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

    pub fn register_object<T: SimConnectObject>(&mut self) -> Result<u32, SimConnectError> {
        let type_id: String = std::any::type_name::<T>().into();

        if self.registered_objects.contains(&type_id) {
            return Err(SimConnectError::ObjectAlreadyRegistered(type_id));
        }

        self.registered_objects.push(type_id.clone());

        let id = self
            .registered_objects
            .iter()
            .position(|p| p == &type_id)
            .ok_or_else(|| {
                SimConnectError::UnexpectedError("failed to find registered event".to_string())
            })?;
        let id = u32::try_from(id)?;

        T::register(self, id)?;

        Ok(id)
    }

    #[tracing::instrument(name = "SimConnect::register_event")]
    pub fn register_event(&self, event: Event) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_MapClientEventToSimEvent(
                self.handle.as_ptr(),
                event as u32,
                event.into_c_char(),
            )
        });

        let group = Group::Group0;

        success!(unsafe {
            bindings::SimConnect_AddClientEventToNotificationGroup(
                self.handle.as_ptr(),
                group as u32,
                event as u32,
                0,
            )
        });

        success!(unsafe {
            bindings::SimConnect_SetNotificationGroupPriority(self.handle.as_ptr(), group as u32, 1)
        });

        Ok(())
    }

    #[tracing::instrument(name = "SimConnect::add_to_data_definition")]
    pub fn add_to_data_definition(
        &self,
        request_id: u32,
        datum_name: &str,
        units_name: &str,
        data_type: DataType,
    ) -> Result<(), SimConnectError> {
        let c_type = match data_type {
            DataType::F64 => bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
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

    #[tracing::instrument(name = "SimConnect::request_data_on_sim_object")]
    pub fn request_data_on_sim_object(
        &self,
        request_id: u32,
        period: PeriodEnum,
        condition: ConditionEnum,
    ) -> Result<(), SimConnectError> {
        unsafe {
            let (simconnect_period, simconnect_interval) = match period {
                PeriodEnum::VisualFrame { interval } => (
                    bindings::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_VISUAL_FRAME,
                    interval,
                ),
                PeriodEnum::Second => (bindings::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SECOND, 0),
            };

            let simconnect_flags: u32 = match condition {
                ConditionEnum::None => 0,
                ConditionEnum::Changed => bindings::SIMCONNECT_DATA_REQUEST_FLAG_CHANGED,
            };

            success!(bindings::SimConnect_RequestDataOnSimObject(
                self.handle.as_ptr(),
                request_id,
                request_id,
                request_id,
                simconnect_period,
                simconnect_flags,
                0,
                simconnect_interval,
                0,
            ));
        }

        Ok(())
    }

    #[tracing::instrument(name = "SimConnect::subscribe_to_airport_list")]
    pub fn subscribe_to_airport_list(&self, request_id: u32) -> Result<(), SimConnectError> {
        unsafe {
            success!(bindings::SimConnect_SubscribeToFacilities(
                self.handle.as_ptr(),
                bindings::SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_AIRPORT,
                request_id,
            ));
        }

        Ok(())
    }

    #[tracing::instrument(name = "SimConnect::request_airport_list")]
    pub fn request_airport_list(&self, request_id: u32) -> Result<(), SimConnectError> {
        unsafe {
            success!(bindings::SimConnect_RequestFacilitiesList(
                self.handle.as_ptr(),
                bindings::SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_AIRPORT,
                request_id,
            ));
        }

        Ok(())
    }

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
                        let data = NotificationData {
                            type_id: object_type.clone(),
                            data_addr: std::ptr::addr_of!(event.dwData),
                        };

                        Some(Notification::Data(data))
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

                let data = event
                    .rgData
                    .iter()
                    .map(|data| AirportData {
                        icao: fixed_c_str_to_string(&data.Icao),
                        lat: data.Latitude,
                        lon: data.Longitude,
                        alt: data.Altitude,
                    })
                    .collect::<Vec<_>>();

                Some(Notification::AirportList(data))
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
}

impl Drop for SimConnect {
    fn drop(&mut self) {
        let _ = unsafe { bindings::SimConnect_Close(self.handle.as_ptr()) };
    }
}
