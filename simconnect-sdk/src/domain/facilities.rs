use crate::bindings;

/// Facility Type. The simulation keeps a facilities cache of all the airports, waypoints, NDB and VOR stations within a certain radius of the user aircraft.
/// They can be requested using [`crate::SimConnect::subscribe_to_facilities`] or [`crate::SimConnect::request_facilities_list`].
#[derive(Debug)]
pub enum FacilityType {
    Airport,
    Waypoint,
    NDB,
    VOR,
}

impl From<FacilityType> for i32 {
    fn from(facility_type: FacilityType) -> Self {
        match facility_type {
            FacilityType::Airport => {
                bindings::SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_AIRPORT
            }
            FacilityType::Waypoint => {
                bindings::SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_WAYPOINT
            }
            FacilityType::NDB => {
                bindings::SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_NDB
            }
            FacilityType::VOR => {
                bindings::SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_VOR
            }
        }
    }
}

impl FacilityType {
    pub fn to_type_name(&self) -> String {
        match self {
            FacilityType::Airport => std::any::type_name::<Airport>(),
            FacilityType::Waypoint => std::any::type_name::<Waypoint>(),
            FacilityType::NDB => std::any::type_name::<NDB>(),
            FacilityType::VOR => std::any::type_name::<VOR>(),
        }
        .into()
    }
}

/// Information on a single airport in the facilities cache.
#[derive(Debug, Clone)]
pub struct Airport {
    /// ICAO of the facility.
    pub icao: String,
    /// Latitude of the airport in facility.
    pub lat: f64,
    /// Longitude of the airport in facility.
    pub lon: f64,
    /// Altitude of the facility in meters.
    pub alt: f64,
}

/// Information on a single waypoint in the facilities cache.
#[derive(Debug, Clone)]
pub struct Waypoint {
    /// ICAO of the facility.
    pub icao: String,
    /// Latitude of the airport in facility.
    pub lat: f64,
    /// Longitude of the airport in facility.
    pub lon: f64,
    /// Altitude of the facility in meters.
    pub alt: f64,
    /// The magnetic variation of the waypoint in degrees.
    pub mag_var: f32,
}

/// Information on a single NDB station in the facilities cache.
#[derive(Debug, Clone)]
pub struct NDB {
    /// ICAO of the facility.
    pub icao: String,
    /// Latitude of the airport in facility.
    pub lat: f64,
    /// Longitude of the airport in facility.
    pub lon: f64,
    /// Altitude of the facility in meters.
    pub alt: f64,
    /// The magnetic variation of the waypoint in degrees.
    pub mag_var: f32,
    /// Frequency of the station in Hz.
    pub frequency: u32,
}

/// Information on a single VOR station in the facilities cache.
#[derive(Debug, Clone)]
pub struct VOR {
    /// ICAO of the facility.
    pub icao: String,
    /// Latitude of the airport in facility.
    pub lat: f64,
    /// Longitude of the airport in facility.
    pub lon: f64,
    /// Altitude of the facility in meters.
    pub alt: f64,
    /// The magnetic variation of the waypoint in degrees.
    pub mag_var: f32,
    /// True if the station has a NAV transmitter, and if so, `glide_lat`, `glide_lon` and `glide_alt` contain valid data.
    pub has_nav_signal: bool,
    /// True if the station transmits an ILS localizer angle, and if so `localizer` contains valid data.
    pub has_localizer: bool,
    /// True if the station transmits an ILS approach angle, and if so `glide_slope_angle` contains valid data.
    pub has_glide_slope: bool,
    /// True if the station transmits a DME signal, and if so the inherited DME fFrequency contains valid data.
    pub has_dme: bool,
    /// The ILS localizer angle in degrees.
    pub localizer: Option<f32>,
    /// The latitude of the glide slope transmitter in degrees.
    pub glide_lat: Option<f64>,
    /// The longitude of the glide slope transmitter in degrees.
    pub glide_lon: Option<f64>,
    /// The altitude of the glide slope transmitter in degrees.
    pub glide_alt: Option<f64>,
    /// The ILS approach angle in degrees.
    pub glide_slope_angle: Option<f32>,
    /// Frequency of the station in Hz.
    pub frequency: Option<u32>,
}
