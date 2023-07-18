use std::os::raw::c_char;

use crate::{bindings, SimConnectError, helpers::{BCD16, bcd16_to_u16, u16_to_bcd16}};

// System Events start from 0 so we have to stagger the values to avoid collisions.
pub(crate) const CLIENT_EVENT_DISCRIMINANT_START: u32 = 1024;

/// SimConnect Client Event Request.
///
/// Defined by <https://www.prepar3d.com/SDKv5/sdk/references/variables/event_ids.html>.
/// Extended by <https://docs.flightsimulator.com/html/Programming_Tools/Event_IDs/Event_IDs.htm>.
#[derive(Debug, Copy, Clone, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum ClientEventRequest {
    // ---------------
    // Aircraft Engine
    // ---------------
    /// Set throttle 1 exactly (0 to 16383).
    Throttle1Set = CLIENT_EVENT_DISCRIMINANT_START,
    /// Set throttle 2 exactly (0 to 16383).
    Throttle2Set,
    /// Set throttle 3 exactly (0 to 16383).
    Throttle3Set,
    /// Set throttle 4 exactly (0 to 16383).
    Throttle4Set,
    // ---------------
    // Aircraft Flight Controls
    // ---------------
    /// Sets elevator position (-16383 - +16383).
    AxisElevatorSet,
    // ---------------
    // Aircraft Miscellaneous Systems
    // ---------------
    /// Increment brake pressure. Note: These are simulated spring-loaded toe brakes, which will bleed back to zero over time.
    Brakes,
    /// Increments left brake pressure. Note: This is a simulated spring-loaded toe brake, which will bleed back to zero over time.
    BrakesLeft,
    /// Increments right brake pressure. Note: This is a simulated spring-loaded toe brake, which will bleed back to zero over time.
    BrakesRight,
    /// Sets left brake position from axis controller (e.g. joystick). -16383 (0 brakes) to +16383 (max brakes).
    AxisLeftBrakeSet,
    /// Sets right brake position from axis controller (e.g. joystick). -16383 (0 brakes) to +16383 (max brakes).
    AxisRightBrakeSet,
    /// Toggles parking brake on/off.
    ParkingBrakes,
    // Comm Radios
    Com1RadioStbySet,
    Com2RadioStbySet,
    Com3RadioStbySet,
    Com1RadioSwap,
    Com2RadioSwap,
    Com3RadioSwap,
    // Nav Radios
    Nav1RadioStbySet,
    Nav2RadioStbySet,
    Nav3RadioStbySet,
    Nav4RadioStbySet,
    Nav1RadioSwap,
    Nav2RadioSwap,
    Nav3RadioSwap,
    Nav4RadioSwap,
    // Transponder
    TransponderSet
}

impl ClientEventRequest {
    pub(crate) fn into_c_char(self) -> *const c_char {
        match self {
            // Aircraft Engine
            Self::Throttle1Set => "THROTTLE1_SET\0".as_ptr() as *const c_char,
            Self::Throttle2Set => "THROTTLE2_SET\0".as_ptr() as *const c_char,
            Self::Throttle3Set => "THROTTLE3_SET\0".as_ptr() as *const c_char,
            Self::Throttle4Set => "THROTTLE4_SET\0".as_ptr() as *const c_char,
            // Aircraft Flight Controls
            Self::AxisElevatorSet => "AXIS_ELEVATOR_SET\0".as_ptr() as *const c_char,
            // Aircraft Miscellaneous Systems
            Self::Brakes => "BRAKES\0".as_ptr() as *const c_char,
            Self::BrakesLeft => "BRAKES_LEFT\0".as_ptr() as *const c_char,
            Self::BrakesRight => "BRAKES_RIGHT\0".as_ptr() as *const c_char,
            Self::AxisLeftBrakeSet => "AXIS_LEFT_BRAKE_SET\0".as_ptr() as *const c_char,
            Self::AxisRightBrakeSet => "AXIS_RIGHT_BRAKE_SET\0".as_ptr() as *const c_char,
            Self::ParkingBrakes => "PARKING_BRAKES\0".as_ptr() as *const c_char,
            // Comm Radios
            Self::Com1RadioStbySet => "COM_STBY_RADIO_SET_HZ\0".as_ptr() as *const c_char,
            Self::Com2RadioStbySet => "COM2_STBY_RADIO_SET_HZ\0".as_ptr() as *const c_char,
            Self::Com3RadioStbySet => "COM3_STBY_RADIO_SET_HZ\0".as_ptr() as *const c_char,
            Self::Com1RadioSwap => "COM_RADIO_SWAP\0".as_ptr() as *const c_char,
            Self::Com2RadioSwap => "COM2_RADIO_SWAP\0".as_ptr() as *const c_char,
            Self::Com3RadioSwap => "COM3_RADIO_SWAP\0".as_ptr() as *const c_char,
            // Nav Radios
            Self::Nav1RadioStbySet => "NAV1_STBY_SET_HZ\0".as_ptr() as *const c_char,
            Self::Nav2RadioStbySet => "NAV2_STBY_SET_HZ\0".as_ptr() as *const c_char,
            Self::Nav3RadioStbySet => "NAV3_STBY_SET_HZ\0".as_ptr() as *const c_char,
            Self::Nav4RadioStbySet => "NAV4_STBY_SET_HZ\0".as_ptr() as *const c_char,
            Self::Nav1RadioSwap => "NAV1_RADIO_SWAP\0".as_ptr() as *const c_char,
            Self::Nav2RadioSwap => "NAV2_RADIO_SWAP\0".as_ptr() as *const c_char,
            Self::Nav3RadioSwap => "NAV3_RADIO_SWAP\0".as_ptr() as *const c_char,
            Self::Nav4RadioSwap => "NAV4_RADIO_SWAP\0".as_ptr() as *const c_char,
            // Transponder
            Self::TransponderSet => "XPNDR_SET\0".as_ptr() as *const c_char
        }
    }
}

/// SimConnect Client Event.
///
/// Defined by <https://www.prepar3d.com/SDKv5/sdk/references/variables/event_ids.html>.
/// Extended by <https://docs.flightsimulator.com/html/Programming_Tools/Event_IDs/Event_IDs.htm>.
#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum ClientEvent {
    // ---------------
    // Aircraft Engine
    // ---------------
    /// Set throttle 1 exactly (0 to 16383).
    Throttle1Set {
        /// -16383 (0 throttle) to +16383 (max throttle).
        value: i32,
    },
    /// Set throttle 2 exactly (0 to 16383).
    Throttle2Set {
        /// -16383 (0 throttle) to +16383 (max throttle).
        value: i32,
    },
    /// Set throttle 3 exactly (0 to 16383).
    Throttle3Set {
        /// -16383 (0 throttle) to +16383 (max throttle).
        value: i32,
    },
    /// Set throttle 4 exactly (0 to 16383).
    Throttle4Set {
        /// -16383 (0 throttle) to +16383 (max throttle).
        value: i32,
    },
    // ---------------
    // Aircraft Flight Controls
    // ---------------
    /// Sets elevator position (-16383 - +16383).
    AxisElevatorSet {
        /// -16383 (full down) to +16383 (full up).
        value: i32,
    },
    // ---------------
    // Aircraft Miscellaneous Systems
    // ---------------
    /// Increment brake pressure. Note: These are simulated spring-loaded toe brakes, which will bleed back to zero over time.
    Brakes,
    /// Increments left brake pressure. Note: This is a simulated spring-loaded toe brake, which will bleed back to zero over time.
    BrakesLeft,
    /// Increments right brake pressure. Note: This is a simulated spring-loaded toe brake, which will bleed back to zero over time.
    BrakesRight,
    /// Sets left brake position from axis controller (e.g. joystick). -16383 (0 brakes) to +16383 (max brakes).
    AxisLeftBrakeSet {
        /// -16383 (0 brakes) to +16383 (max brakes).
        value: i32,
    },
    /// Sets right brake position from axis controller (e.g. joystick). -16383 (0 brakes) to +16383 (max brakes).
    AxisRightBrakeSet {
        /// -16383 (0 brakes) to +16383 (max brakes).
        value: i32,
    },
    /// Toggles parking brake on/off.
    ParkingBrakes,
    /// Aircraft Radio Navigation Events
    /// https://docs.flightsimulator.com/html/Programming_Tools/Event_IDs/Aircraft_Radio_Navigation_Events.htm
    /// Set COM 1 radio frequency.
    Com1RadioStbySet {
        /// Value is desired frequency eg. 127.65
        value: f32,
    },
    /// Set COM 2 radio frequency.
    Com2RadioStbySet {
        /// Value is desired frequency eg. 127.65
        value: f32,
    },
    /// Set COM 2 radio frequency.
    Com3RadioStbySet {
        /// Value is desired frequency eg. 127.65
        value: f32,
    },
    /// Swap COM 1 standby to active frequency.
    Com1RadioSwap,
    /// Swap COM 2 standby to active frequency.
    Com2RadioSwap,
    /// Swap COM 3 standby to active frequency.
    Com3RadioSwap,
    // Nav Radios
    Nav1RadioStbySet {
        /// Value is desired frequency eg. 127.65
        value: f32,
    },
    Nav2RadioStbySet {
        /// Value is desired frequency eg. 127.65
        value: f32,
    },
    Nav3RadioStbySet {
        /// Value is desired frequency eg. 127.65
        value: f32,
    },
    Nav4RadioStbySet {
        /// Value is desired frequency eg. 127.65
        value: f32,
    },
    /// Swap NAV 1 standby to active frequency.
    Nav1RadioSwap,
    /// Swap NAV 2 standby to active frequency.
    Nav2RadioSwap,
    /// Swap NAV 3 standby to active frequency.
    Nav3RadioSwap,
    /// Swap NAV 4 standby to active frequency.
    Nav4RadioSwap,
    /// Sets the Transponder value.
    TransponderSet {
        /// Transponder value eg. 0174
        /// Largest individual digit is 7, transponder values will overflow.
        value: u16,
    }
}

impl TryFrom<&bindings::SIMCONNECT_RECV_EVENT> for ClientEvent {
    type Error = SimConnectError;

    fn try_from(event: &bindings::SIMCONNECT_RECV_EVENT) -> Result<Self, Self::Error> {
        let request = ClientEventRequest::try_from(event.uEventID)
            .map_err(|_| SimConnectError::UnimplementedEventType(event.uEventID))?;

        match request {
            // Aircraft Engine
            ClientEventRequest::Throttle1Set => Ok(Self::Throttle1Set {
                value: event.dwData as i32,
            }),
            ClientEventRequest::Throttle2Set => Ok(Self::Throttle2Set {
                value: event.dwData as i32,
            }),
            ClientEventRequest::Throttle3Set => Ok(Self::Throttle3Set {
                value: event.dwData as i32,
            }),
            ClientEventRequest::Throttle4Set => Ok(Self::Throttle4Set {
                value: event.dwData as i32,
            }),
            // Aircraft Flight Controls
            ClientEventRequest::AxisElevatorSet => Ok(Self::AxisElevatorSet {
                value: event.dwData as i32,
            }),
            // Aircraft Miscellaneous Systems
            ClientEventRequest::Brakes => Ok(Self::Brakes),
            ClientEventRequest::BrakesLeft => Ok(Self::BrakesLeft),
            ClientEventRequest::BrakesRight => Ok(Self::BrakesRight),
            ClientEventRequest::AxisLeftBrakeSet => Ok(Self::AxisLeftBrakeSet {
                value: event.dwData as i32,
            }),
            ClientEventRequest::AxisRightBrakeSet => Ok(Self::AxisRightBrakeSet {
                value: event.dwData as i32,
            }),
            ClientEventRequest::ParkingBrakes => Ok(Self::ParkingBrakes),
            // Comm Radios
            ClientEventRequest::Com1RadioStbySet => Ok(Self::Com1RadioStbySet {
                value: (event.dwData as f32 / 1000000.0),
            }),
            ClientEventRequest::Com2RadioStbySet => Ok(Self::Com2RadioStbySet {
                value: (event.dwData as f32 / 1000000.0),
            }),
            ClientEventRequest::Com3RadioStbySet => Ok(Self::Com3RadioStbySet {
                value: (event.dwData as f32 / 1000000.0),
            }),
            ClientEventRequest::Com1RadioSwap => Ok(Self::Com1RadioSwap),
            ClientEventRequest::Com2RadioSwap => Ok(Self::Com2RadioSwap),
            ClientEventRequest::Com3RadioSwap => Ok(Self::Com3RadioSwap),
            // Nav Radios
            ClientEventRequest::Nav1RadioStbySet => Ok(Self::Nav1RadioStbySet {
                value: (event.dwData as f32 / 1000000.0),
            }),
            ClientEventRequest::Nav2RadioStbySet => Ok(Self::Nav2RadioStbySet {
                value: (event.dwData as f32 / 1000000.0),
            }),
            ClientEventRequest::Nav3RadioStbySet => Ok(Self::Nav3RadioStbySet {
                value: (event.dwData as f32 / 1000000.0),
            }),
            ClientEventRequest::Nav4RadioStbySet => Ok(Self::Nav4RadioStbySet {
                value: (event.dwData as f32 / 1000000.0),
            }),
            ClientEventRequest::Nav1RadioSwap => Ok(Self::Nav1RadioSwap),
            ClientEventRequest::Nav2RadioSwap => Ok(Self::Nav2RadioSwap),
            ClientEventRequest::Nav3RadioSwap => Ok(Self::Nav3RadioSwap),
            ClientEventRequest::Nav4RadioSwap => Ok(Self::Nav4RadioSwap),
            // Transponder
            ClientEventRequest::TransponderSet => Ok(Self::TransponderSet {
                value: bcd16_to_u16(event.dwData as u16),
            })
        }
    }
}

impl From<ClientEvent> for (ClientEventRequest, u32) {
    fn from(event: ClientEvent) -> Self {
        match event {
            // Aircraft Engine
            ClientEvent::Throttle1Set { value } => (ClientEventRequest::Throttle1Set, value as u32),
            ClientEvent::Throttle2Set { value } => (ClientEventRequest::Throttle2Set, value as u32),
            ClientEvent::Throttle3Set { value } => (ClientEventRequest::Throttle3Set, value as u32),
            ClientEvent::Throttle4Set { value } => (ClientEventRequest::Throttle4Set, value as u32),
            // Aircraft Flight Controls
            ClientEvent::AxisElevatorSet { value } => (ClientEventRequest::AxisElevatorSet, value as u32),
            // Aircraft Miscellaneous Systems
            ClientEvent::Brakes => (ClientEventRequest::Brakes, 0),
            ClientEvent::BrakesLeft => (ClientEventRequest::BrakesLeft, 0),
            ClientEvent::BrakesRight => (ClientEventRequest::BrakesRight, 0),
            ClientEvent::AxisLeftBrakeSet { value } => (ClientEventRequest::AxisLeftBrakeSet, value as u32),
            ClientEvent::AxisRightBrakeSet { value } => (ClientEventRequest::AxisRightBrakeSet, value as u32),
            ClientEvent::ParkingBrakes => (ClientEventRequest::ParkingBrakes, 0),
            // Comm Radios
            ClientEvent::Com1RadioStbySet { value } => (ClientEventRequest::Com1RadioStbySet, (value * 1000000.0) as u32 ),
            ClientEvent::Com2RadioStbySet { value } => (ClientEventRequest::Com2RadioStbySet, (value * 1000000.0) as u32),
            ClientEvent::Com3RadioStbySet { value } => (ClientEventRequest::Com3RadioStbySet, (value * 1000000.0) as u32),
            ClientEvent::Com1RadioSwap => (ClientEventRequest::Com1RadioSwap, 0),
            ClientEvent::Com2RadioSwap => (ClientEventRequest::Com2RadioSwap, 0),
            ClientEvent::Com3RadioSwap => (ClientEventRequest::Com3RadioSwap, 0),
            // Nav Radios
            ClientEvent::Nav1RadioStbySet { value } => (ClientEventRequest::Nav1RadioStbySet, (value * 1000000.0) as u32),
            ClientEvent::Nav2RadioStbySet { value } => (ClientEventRequest::Nav2RadioStbySet, (value * 1000000.0) as u32),
            ClientEvent::Nav3RadioStbySet { value } => (ClientEventRequest::Nav3RadioStbySet, (value * 1000000.0) as u32),
            ClientEvent::Nav4RadioStbySet { value } => (ClientEventRequest::Nav4RadioStbySet, (value * 1000000.0) as u32),
            ClientEvent::Nav1RadioSwap => (ClientEventRequest::Nav1RadioSwap, 0),
            ClientEvent::Nav2RadioSwap => (ClientEventRequest::Nav2RadioSwap, 0),
            ClientEvent::Nav3RadioSwap => (ClientEventRequest::Nav3RadioSwap, 0),
            ClientEvent::Nav4RadioSwap => (ClientEventRequest::Nav4RadioSwap, 0),
            // Transponder
            ClientEvent::TransponderSet { value } => (ClientEventRequest::TransponderSet, u16_to_bcd16(value) as u32)
        }
    }
}

/// SimConnect Event Flags for TransmitClientEvent
/// 
/// https://www.prepar3d.com/SDKv5/sdk/simconnect_api/references/general_functions.html#SimConnect_TransmitClientEvent
#[derive(Debug, Copy, Clone, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u32)]
pub enum EventFlag {
    DoNothing,
    SlowRepeatTimer,
    FastRepeatTimer,
    // GroupIdIsPriority Not currently implemented in SDK
}