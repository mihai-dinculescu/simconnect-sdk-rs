#![allow(unused_variables, dead_code)]
use simconnect_sdk_derive::SimConnectObject;

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = 123, condition = "none")]
struct GpsData1 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = 123)]
struct GpsData2 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "X")]
struct GpsData3 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "X")]
struct GpsData4 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", interval = "X")]
struct GpsData5 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", interval = 0.0)]
struct GpsData6 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct GpsData7 {
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
    pub lat: String,
}

fn main() {}
