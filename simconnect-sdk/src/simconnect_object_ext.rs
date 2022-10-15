use crate::{Object, SimConnect, SimConnectError};

/// Trait to be implemented by objects that can be registered with SimConnect.
pub trait SimConnectObjectExt: Clone + for<'a> TryFrom<&'a Object> {
    fn register(client: &mut SimConnect, id: u32) -> Result<(), SimConnectError>;
}
