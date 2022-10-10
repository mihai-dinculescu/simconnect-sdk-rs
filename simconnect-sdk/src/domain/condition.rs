/// Specifies under which conditions the data is to be sent by the server and received by the client.
#[derive(Debug, Clone)]
pub enum Condition {
    /// The default, data will be sent strictly according to the defined period.
    None,
    /// Data will only be sent to the client when one or more values have changed.
    Changed,
}
