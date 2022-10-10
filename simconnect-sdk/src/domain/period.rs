/// Specifies how often the data is to be sent by the server and received by the client.
/// 0 - every interval.
/// 1 - every other interval.
/// 2 - every third interval.
/// etc.
#[derive(Debug, Clone)]
pub enum Period {
    /// Specifies that the data should be sent once only. Note that this is not an efficient way of receiving data frequently, use one of the other periods if there is a regular frequency to the data request.
    Once,
    /// Specifies that the data should be sent every visual (rendered) frame.
    VisualFrame,
    /// Specifies that the data should be sent every simulated frame, whether that frame is rendered or not.
    SimFrame,
    /// Specifies that the data should be sent once every second.
    Second,
}
