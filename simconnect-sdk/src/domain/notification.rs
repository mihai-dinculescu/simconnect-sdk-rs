use crate::{Airport, Event, SimConnectError, SimConnectObjectExt, Waypoint, NDB, VOR};

/// Notification received from SimConnect.
#[derive(Debug)]
pub enum Notification {
    /// SimConnect open
    Open,
    /// SimConnect event
    Event(Event),
    /// SimConnect object
    Object(Object),
    /// A list of [crate::Airport].
    AirportList(Vec<Airport>),
    /// A list of [crate::Waypoint].
    WaypointList(Vec<Waypoint>),
    /// A list of [crate::NDB].
    NdbList(Vec<NDB>),
    /// A list of [crate::VOR].
    VorList(Vec<VOR>),
    /// SimConnect quit
    Quit,
    /// SimConnect exception
    Exception(u32),
}

/// Notification data object.
#[derive(Debug)]
pub struct Object {
    pub(crate) type_name: String,
    pub(crate) data_addr: *const u32,
}

impl Object {
    pub fn try_transmute<T: SimConnectObjectExt>(&self) -> Result<T, SimConnectError> {
        let type_name: String = std::any::type_name::<T>().into();

        if self.type_name == type_name {
            let data: &T = unsafe { std::mem::transmute_copy(&self.data_addr) };
            Ok(data.clone())
        } else {
            Err(SimConnectError::ObjectMismatch {
                actual: self.type_name.clone(),
                expected: type_name,
            })
        }
    }
}
