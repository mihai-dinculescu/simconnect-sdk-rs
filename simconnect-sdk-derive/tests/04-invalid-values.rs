#![allow(unused_variables, dead_code)]
use simconnect_sdk_derive::SimConnectObject;

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = 123, condition = "none")]
struct Data1 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = 123)]
struct Data2 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "X")]
struct Data3 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "X")]
struct Data4 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", interval = "X")]
struct Data5 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", interval = 0.0)]
struct Data6 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none")]
struct Data7 {
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
    pub lat: u64,
}

fn main() {}
