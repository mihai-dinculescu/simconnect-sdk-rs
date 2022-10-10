#![allow(unused_variables, dead_code)]
use simconnect_sdk_derive::SimConnectObject;

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct GpsData1 {
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct GpsData2 {
    #[simconnect]
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct GpsData3 {
    #[simconnect()]
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct GpsData4 {
    #[simconnect(name = "PLANE LATITUDE")]
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct GpsData5 {
    #[simconnect(unit = "degrees")]
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct GpsData6 {
    #[simconnect(name = "PLANE LATITUDE", name = "PLANE LATITUDE")]
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct GpsData7 {
    #[simconnect(unit = "degrees", unit = "degrees")]
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct GpsData8 {
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees", unit = "degrees")]
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct GpsData9 {
    #[simconnect(nameX = "PLANE LATITUDE", unit = "degrees")]
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct GpsData10 {
    #[simconnect(name = "PLANE LATITUDE", unitX = "degrees")]
    pub lat: f64,
}

fn main() {}
