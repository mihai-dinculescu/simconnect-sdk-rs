#![allow(unused_variables, dead_code)]
use simconnect_sdk_derive::SimConnectObject;

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct Data1 {
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct Data2 {
    #[simconnect]
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct Data3 {
    #[simconnect()]
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct Data4 {
    #[simconnect(unit = "degrees")]
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct Data5 {
    #[simconnect(name = "PLANE LATITUDE", name = "PLANE LATITUDE")]
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct Data6 {
    #[simconnect(unit = "degrees", unit = "degrees")]
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct Data7 {
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees", unit = "degrees")]
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct Data8 {
    #[simconnect(nameX = "PLANE LATITUDE", unit = "degrees")]
    pub lat: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct Data9 {
    #[simconnect(name = "PLANE LATITUDE", unitX = "degrees")]
    pub lat: f64,
}

fn main() {}
