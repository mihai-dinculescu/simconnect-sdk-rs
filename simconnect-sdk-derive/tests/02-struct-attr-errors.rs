#![allow(unused_variables, dead_code)]
use simconnect_sdk_derive::SimConnectObject;

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "visual-frame", condition = "changed")]
struct Data1(f64);

#[derive(Debug, Clone, SimConnectObject)]
struct Data2 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect]
struct Data3 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect()]
struct Data4 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", period = "second")]
struct Data5 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "none", condition = "none")]
struct Data6 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", interval = 0, interval = 0)]
struct Data7 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", test = "test")]
struct Data8 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(periodX = "second", condition = "none")]
struct Data9 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", conditionX = "none")]
struct Data10 {}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", intervalX = 0)]
struct Data11 {}

fn main() {}
