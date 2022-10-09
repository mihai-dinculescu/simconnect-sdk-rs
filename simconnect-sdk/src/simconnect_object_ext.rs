use crate::{NotificationData, SimConnect, SimConnectError};

pub trait SimConnectObjectExt: Clone + for<'a> TryFrom<&'a NotificationData> {
    fn register(client: &mut SimConnect, id: u32) -> Result<(), SimConnectError>;
}
