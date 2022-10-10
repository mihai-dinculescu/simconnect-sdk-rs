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
#[simconnect(condition = "none", condition = "none")]
struct GpsData6 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none", test = "test")]
struct GpsData7 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(periodX = "second", condition = "none")]
struct GpsData8 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", conditionX = "none")]
struct GpsData9 {}

fn main() {}
