use crate::{AirportData, Event, SimConnectError, SimConnectObjectExt};

/// Notification received from SimConnect.
pub enum Notification {
    /// SimConnect open
    Open,
    /// SimConnect event
    Event(Event),
    /// SimConnect object
    Data(NotificationData),
    /// SimConnect airport list
    AirportList(Vec<AirportData>),
    /// SimConnect quit
    Quit,
    /// SimConnect exception
    Exception(u32),
}

/// Notification data object.
pub struct NotificationData {
    pub(crate) type_id: String,
    pub(crate) data_addr: *const u32,
}

impl NotificationData {
    pub fn try_transmute<T: SimConnectObjectExt>(&self) -> Result<T, SimConnectError> {
        let type_id: String = std::any::type_name::<T>().into();

        if self.type_id == type_id {
            let data: &T = unsafe { std::mem::transmute_copy(&self.data_addr) };
            Ok(data.clone())
        } else {
            Err(SimConnectError::ObjectMismatch {
                actual: self.type_id.clone(),
                expected: type_id,
            })
        }
    }
}
