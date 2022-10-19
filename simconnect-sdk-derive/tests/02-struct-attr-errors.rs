#![allow(unused_variables, dead_code)]
use simconnect_sdk_derive::SimConnectObject;

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "visual-frame", condition = "changed")]
struct GpsData1(f64);

#[derive(Debug, Clone, SimConnectObject)]
struct GpsData2 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect]
struct GpsData3 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect()]
struct GpsData4 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", period = "second")]
struct GpsData5 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none", condition = "none")]
struct GpsData6 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", interval = 0, interval = 0)]
struct GpsData7 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", test = "test")]
struct GpsData8 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(periodX = "second", condition = "none")]
struct GpsData9 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", conditionX = "none")]
struct GpsData10 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", intervalX = 0)]
struct GpsData11 {}

fn main() {}
