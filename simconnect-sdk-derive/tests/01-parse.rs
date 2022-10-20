#![allow(unused_variables, dead_code)]

use simconnect_sdk_derive::SimConnectObject;
#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second")]
struct Data1 {
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
    pub lat: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
    pub lon: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct Data2 {
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
    pub lat: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
    pub lon: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "visual-frame", condition = "changed")]
struct Data3 {
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
    pub lat: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
    pub lon: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "visual-frame", condition = "changed", interval = 0)]
struct Data4 {
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
    pub lat: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
    pub lon: f64,
}

fn main() {}
