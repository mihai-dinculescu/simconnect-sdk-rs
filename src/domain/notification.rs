use crate::{AirportData, Event, SimConnectObject};

pub enum Notification {
    Open,
    Event(Event),
    Data(NotificationData),
    AirportList(Vec<AirportData>),
    Quit,
    Exception(u32),
}

pub struct NotificationData {
    pub(crate) type_id: String,
    pub(crate) data_addr: *const u32,
}

impl NotificationData {
    pub fn try_into<T: SimConnectObject>(&self) -> Option<T> {
        let type_id: String = std::any::type_name::<T>().into();

        if self.type_id == type_id {
            let data: &T = unsafe { std::mem::transmute_copy(&self.data_addr) };
            Some(data.clone())
        } else {
            None
        }
    }
}
