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
#[simconnect(period = "second", condition = "none")]
struct GpsData5 {
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
    pub lat: String,
}

fn main() {}
