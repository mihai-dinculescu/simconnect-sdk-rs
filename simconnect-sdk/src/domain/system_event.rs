use std::os::raw::c_char;

use crate::{bindings, fixed_c_str_to_string, SimConnectError};

/// SimConnect System Event Request.
#[derive(Debug, Copy, Clone, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum SystemEventRequest {
    /// Request a notification every second.
    OneSecond = 0,
    /// Request a notification every four seconds.
    FourSeconds,
    /// Request notifications six times per second. This is the same rate that joystick movement events are transmitted.
    SixTimesPerSecond,
    /// Request a notification when the aircraft flight dynamics file is changed. These files have a .AIR extension. The filename is returned.
    AircraftLoaded,
    /// Request a notification if the user aircraft crashes.
    Crashed,
    /// Request a notification when the crash cut-scene has completed.
    CrashReset,
    /// Request a notification when a flight is loaded. Note that when a flight is ended, a default flight is typically loaded, so these events will occur when flights and missions are started and finished. The filename of the flight loaded is returned.
    FlightLoaded,
    /// Request a notification when a flight is saved correctly. The filename of the flight saved is returned.
    FlightSaved,
    /// Request a notification when a new flight plan is activated. The filename of the activated flight plan is returned.
    FlightPlanActivated,
    /// Request a notification when the active flight plan is de-activated.
    FlightPlanDeactivated,
    /// Request notifications every visual frame.
    Frame,
    /// Request notifications when the scenario is paused or unpaused, and also immediately returns the current pause state.
    Pause,
    /// Request a notification when the scenario is paused.
    Paused,
    /// Request notifications for every visual frame that the simulation is paused.
    PauseFrame,
    /// Request a notification when the user changes the position of their aircraft through a dialog.
    PositionChanged,
    /// Request notifications when the scenario is running or not, and also immediately returns the current state.
    Sim,
    /// The simulator is running. Typically the user is actively controlling the vehicle which is on the ground, underwater or in the air.
    SimStart,
    /// The simulator is not running. Typically the user is loading a scenario, navigating the user interface or in a dialog.
    SimStop,
    /// Requests a notification when the master sound switch is changed. This request will also return the current state of the master sound switch immediately.
    Sound,
    /// Request a notification when the flight is un-paused.
    Unpaused,
    /// Requests a notification when the user aircraft view is changed. This request will also return the current view immediately.
    View,
}

impl SystemEventRequest {
    pub(crate) fn into_c_char(self) -> *const c_char {
        match self {
            SystemEventRequest::OneSecond => c"1sec".as_ptr() as *const c_char,
            SystemEventRequest::FourSeconds => c"4sec".as_ptr() as *const c_char,
            SystemEventRequest::SixTimesPerSecond => c"6Hz".as_ptr() as *const c_char,
            SystemEventRequest::AircraftLoaded => c"AircraftLoaded".as_ptr() as *const c_char,
            SystemEventRequest::Crashed => c"Crashed".as_ptr() as *const c_char,
            SystemEventRequest::CrashReset => c"CrashReset".as_ptr() as *const c_char,
            SystemEventRequest::FlightLoaded => c"FlightLoaded".as_ptr() as *const c_char,
            SystemEventRequest::FlightSaved => c"FlightSaved".as_ptr() as *const c_char,
            SystemEventRequest::FlightPlanActivated => {
                c"FlightPlanActivated".as_ptr() as *const c_char
            }
            SystemEventRequest::FlightPlanDeactivated => {
                c"FlightPlanDeactivated".as_ptr() as *const c_char
            }
            SystemEventRequest::Frame => c"Frame".as_ptr() as *const c_char,
            SystemEventRequest::Pause => c"Pause".as_ptr() as *const c_char,
            SystemEventRequest::Paused => c"Paused".as_ptr() as *const c_char,
            SystemEventRequest::PauseFrame => c"PauseFrame".as_ptr() as *const c_char,
            SystemEventRequest::PositionChanged => c"PositionChanged".as_ptr() as *const c_char,
            SystemEventRequest::Sim => c"Sim".as_ptr() as *const c_char,
            SystemEventRequest::SimStart => c"SimStart".as_ptr() as *const c_char,
            SystemEventRequest::SimStop => c"SimStop".as_ptr() as *const c_char,
            SystemEventRequest::Sound => c"Sound".as_ptr() as *const c_char,
            SystemEventRequest::Unpaused => c"Unpaused".as_ptr() as *const c_char,
            SystemEventRequest::View => c"View".as_ptr() as *const c_char,
        }
    }
}

/// Cockpit view type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u32)]
pub enum ViewType {
    /// No cockpit view.
    None = 0,
    /// 2D Panels in cockpit view.
    Cockpit2D = bindings::SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_COCKPIT_2D,
    /// Virtual (3D) panels in cockpit view.
    CockpitVirtual = bindings::SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_COCKPIT_VIRTUAL,
    /// Orthogonal (map) view.
    Orthogonal = bindings::SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_ORTHOGONAL,
}

/// SimConnect System Event Notification.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum SystemEvent {
    /// A notification every second.
    OneSecond,
    /// A notification every four seconds.
    FourSeconds,
    /// A notification six times per second. This is the same rate that joystick movement events are transmitted.
    SixTimesPerSecond,
    /// A notification when the aircraft flight dynamics file is changed. These files have a .AIR extension. The filename is returned.
    AircraftLoaded {
        /// The returned filename.
        file_name: String,
    },
    /// A notification if the user aircraft crashes.
    Crashed,
    /// A notification when the crash cut-scene has completed.
    CrashReset,
    /// A notification when a flight is loaded. Note that when a flight is ended, a default flight is typically loaded, so these events will occur when flights and missions are started and finished. The filename of the flight loaded is returned.
    FlightLoaded {
        /// The returned filename.
        file_name: String,
    },
    /// A notification when a flight is saved correctly. The filename of the flight saved is returned.
    FlightSaved {
        /// The returned filename.
        file_name: String,
    },
    /// A notification when a new flight plan is activated. The filename of the activated flight plan is returned.
    FlightPlanActivated {
        /// The returned filename.
        file_name: String,
    },
    /// A notification when the active flight plan is de-activated.
    FlightPlanDeactivated,
    /// Notifications every visual frame.
    Frame {
        /// The visual frame rate in frames per second.
        frame_rate: f32,
        /// The simulation rate. For example if the simulation is running at four times normal speed -- 4X -- then `4.0` will be returned.
        sim_speed: f32,
    },
    /// Notifications when the scenario is paused or unpaused, and also immediately returns the current pause state.
    Pause {
        /// The current pause state (`true` = paused or `false` = unpaused).
        state: bool,
    },
    /// A notification when the scenario is paused.
    Paused,
    /// Notifications for every visual frame that the simulation is paused.
    PauseFrame {
        /// The visual frame rate in frames per second.
        frame_rate: f32,
        /// The simulation rate. For example if the simulation is running at four times normal speed -- 4X -- then 4.0 will be returned.
        sim_speed: f32,
    },
    /// A notification when the user changes the position of their aircraft through a dialog.
    PositionChanged,
    /// Notifications when the scenario is running or not, and also immediately returns the current state.
    Sim {
        /// The current state (`true` = running or `false` = not running).
        state: bool,
    },
    /// The simulator is running. Typically the user is actively controlling the vehicle which is on the ground, underwater or in the air.
    SimStart,
    /// The simulator is not running. Typically the user is loading a scenario, navigating the user interface or in a dialog.
    SimStop,
    /// A notification when the master sound switch is changed. This request will also return the current state of the master sound switch immediately.
    Sound {
        /// The current state of the master sound switch. `false` if the switch is off, `true` if the switch is on.
        state: bool,
    },
    /// A notification when the flight is un-paused.
    Unpaused,
    /// A notification when the user aircraft view is changed. This request will also return the current view immediately.
    View {
        /// The current cockpit view type.
        view: ViewType,
    },
}

impl TryFrom<&bindings::SIMCONNECT_RECV_EVENT> for SystemEvent {
    type Error = SimConnectError;

    fn try_from(event: &bindings::SIMCONNECT_RECV_EVENT) -> Result<Self, Self::Error> {
        let request = SystemEventRequest::try_from(event.uEventID)
            .map_err(|_| SimConnectError::UnimplementedEventType(event.uEventID))?;

        match request {
            SystemEventRequest::OneSecond => Ok(SystemEvent::OneSecond),
            SystemEventRequest::FourSeconds => Ok(SystemEvent::FourSeconds),
            SystemEventRequest::SixTimesPerSecond => Ok(SystemEvent::SixTimesPerSecond),
            SystemEventRequest::Crashed => Ok(SystemEvent::Crashed),
            SystemEventRequest::CrashReset => Ok(SystemEvent::CrashReset),
            SystemEventRequest::FlightPlanDeactivated => Ok(SystemEvent::FlightPlanDeactivated),
            SystemEventRequest::Pause => Ok(SystemEvent::Pause {
                state: event.dwData == 1,
            }),
            SystemEventRequest::Paused => Ok(SystemEvent::Paused),
            SystemEventRequest::PositionChanged => Ok(SystemEvent::PositionChanged),
            SystemEventRequest::Sim => Ok(SystemEvent::Sim {
                state: event.dwData == 1,
            }),
            SystemEventRequest::SimStart => Ok(SystemEvent::SimStart),
            SystemEventRequest::SimStop => Ok(SystemEvent::SimStop),
            SystemEventRequest::Sound => Ok(SystemEvent::Sound {
                state: event.dwData == bindings::SIMCONNECT_SOUND_SYSTEM_EVENT_DATA_MASTER,
            }),
            SystemEventRequest::Unpaused => Ok(SystemEvent::Unpaused),
            SystemEventRequest::View => Ok(SystemEvent::View {
                view: ViewType::try_from(event.dwData).unwrap_or(ViewType::None),
            }),
            _ => Err(SimConnectError::UnimplementedEventType(event.uEventID)),
        }
    }
}

impl TryFrom<&bindings::SIMCONNECT_RECV_EVENT_FILENAME> for SystemEvent {
    type Error = SimConnectError;

    fn try_from(event: &bindings::SIMCONNECT_RECV_EVENT_FILENAME) -> Result<Self, Self::Error> {
        let request = SystemEventRequest::try_from(event._base.uEventID)
            .map_err(|_| SimConnectError::UnimplementedEventType(event._base.uEventID))?;

        match request {
            SystemEventRequest::AircraftLoaded => {
                let file_name = fixed_c_str_to_string(&event.szFileName);
                Ok(SystemEvent::AircraftLoaded { file_name })
            }
            SystemEventRequest::FlightLoaded => {
                let file_name = fixed_c_str_to_string(&event.szFileName);
                Ok(SystemEvent::FlightLoaded { file_name })
            }
            SystemEventRequest::FlightSaved => {
                let file_name = fixed_c_str_to_string(&event.szFileName);
                Ok(SystemEvent::FlightSaved { file_name })
            }
            SystemEventRequest::FlightPlanActivated => {
                let file_name = fixed_c_str_to_string(&event.szFileName);
                Ok(SystemEvent::FlightPlanActivated { file_name })
            }
            _ => Err(SimConnectError::UnimplementedEventType(
                event._base.uEventID,
            )),
        }
    }
}

impl TryFrom<&bindings::SIMCONNECT_RECV_EVENT_FRAME> for SystemEvent {
    type Error = SimConnectError;

    fn try_from(event: &bindings::SIMCONNECT_RECV_EVENT_FRAME) -> Result<Self, Self::Error> {
        let request = SystemEventRequest::try_from(event._base.uEventID)
            .map_err(|_| SimConnectError::UnimplementedEventType(event._base.uEventID))?;

        match request {
            SystemEventRequest::Frame => Ok(SystemEvent::Frame {
                frame_rate: event.fFrameRate,
                sim_speed: event.fSimSpeed,
            }),
            SystemEventRequest::PauseFrame => Ok(SystemEvent::PauseFrame {
                frame_rate: event.fFrameRate,
                sim_speed: event.fSimSpeed,
            }),
            _ => Err(SimConnectError::UnimplementedEventType(
                event._base.uEventID,
            )),
        }
    }
}
