use crate::{NotificationData, SimConnect, SimConnectError};

pub trait SimConnectObject: Clone + for<'a> TryFrom<&'a NotificationData> {
    fn register(client: &mut SimConnect, id: u32) -> Result<(), SimConnectError>;
}
