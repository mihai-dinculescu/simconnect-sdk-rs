use crate::{
    Airport, ClientEvent, SimConnectError, SimConnectObjectExt, SystemEvent, Waypoint, NDB, VOR,
};

/// Notification received from SimConnect.
#[derive(Debug)]
#[non_exhaustive]
pub enum Notification {
    /// SimConnect open
    Open,
    /// SimConnect client event
    ClientEvent(ClientEvent),
    /// SimConnect system event
    SystemEvent(SystemEvent),
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
}

/// Notification data object.
#[derive(Debug)]
pub struct Object {
    pub(crate) type_name: String,
    pub(crate) data_addr: *const u32,
}

impl Object {
    /// Try and transmute this SimConnect object as a `T` struct.
    ///
    /// # Errors
    /// - [`crate::SimConnectError::ObjectMismatch`] -- The type of this SimConnect object is different from `T`.
    pub fn try_transmute<T: SimConnectObjectExt, I>(&self) -> Result<I, SimConnectError> {
        let type_name: String = std::any::type_name::<T>().into();

        if self.type_name == type_name {
            let data: I = unsafe { std::ptr::read_unaligned(self.data_addr as *const I) };
            Ok(data)
        } else {
            Err(SimConnectError::ObjectMismatch {
                actual: self.type_name.clone(),
                expected: type_name,
            })
        }
    }
}
