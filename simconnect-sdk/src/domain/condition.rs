use crate::bindings;

/// Specifies under which conditions the data is to be sent by the server and received by the client.
#[derive(Debug)]
pub enum Condition {
    /// The default, data will be sent strictly according to the defined period.
    None,
    /// Data will only be sent to the client when one or more values have changed. All the variables in a data definition will be returned if just one of the values changes.
    Changed,
}

impl From<Condition> for u32 {
    fn from(condition: Condition) -> Self {
        match condition {
            Condition::None => 0,
            Condition::Changed => bindings::SIMCONNECT_DATA_REQUEST_FLAG_CHANGED,
        }
    }
}
