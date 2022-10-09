use std::os::raw::c_char;

#[derive(Debug, Copy, Clone, num_enum::TryFromPrimitive)]
#[repr(u32)]
pub enum Event {
    Brakes,
    BrakesLeft,
    AxisLeftBrakeSet,
}

impl Event {
    pub(crate) fn into_c_char(self) -> *const c_char {
        match self {
            Event::Brakes => "BRAKES\0".as_ptr() as *const c_char,
            Event::BrakesLeft => "BRAKES_LEFT\0".as_ptr() as *const c_char,
            Event::AxisLeftBrakeSet => "AXIS_LEFT_BRAKE_SET\0".as_ptr() as *const c_char,
        }
    }
}
