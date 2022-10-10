#![allow(dead_code)]

use simconnect_sdk::{
    Condition, DataType, Notification, NotificationData, Period, SimConnect, SimConnectError,
    SimConnectObjectExt,
};
use tracing::{error, info};

use simconnect_sdk_examples::setup_logging;

#[derive(Debug, Clone)]
pub struct GpsData {
    lat: f64,
    lon: f64,
    alt: f64,
    gps_ground_magnetic_track: f64,
    gps_ground_speed: f64,
}

impl SimConnectObjectExt for GpsData {
    fn register(client: &mut SimConnect, id: u32) -> Result<(), SimConnectError> {
        client.add_to_data_definition(id, "PLANE LATITUDE", "degrees", DataType::Float64)?;
        client.add_to_data_definition(id, "PLANE LONGITUDE", "degrees", DataType::Float64)?;
        client.add_to_data_definition(id, "PLANE ALTITUDE", "meters", DataType::Float64)?;
        client.add_to_data_definition(
            id,
            "GPS GROUND MAGNETIC TRACK",
            "degrees",
            DataType::Float64,
        )?;
        client.add_to_data_definition(
            id,
            "GPS GROUND SPEED",
            "Meters per second",
            DataType::Float64,
        )?;
        client.request_data_on_sim_object(id, Period::Second, Condition::None, 0)?;

        Ok(())
    }
}

impl TryFrom<&NotificationData> for GpsData {
    type Error = SimConnectError;

    fn try_from(value: &NotificationData) -> Result<Self, Self::Error> {
        value.try_transmute::<GpsData>()
    }
}

#[derive(Debug, Clone)]
pub struct OnGround {
    sim_on_ground: bool,
}

impl SimConnectObjectExt for OnGround {
    fn register(client: &mut SimConnect, id: u32) -> Result<(), SimConnectError> {
        client.add_to_data_definition(id, "SIM ON GROUND", "bool", DataType::Bool)?;
        client.request_data_on_sim_object(id, Period::Second, Condition::Changed, 0)?;

        Ok(())
    }
}

impl TryFrom<&NotificationData> for OnGround {
    type Error = SimConnectError;

    fn try_from(value: &NotificationData) -> Result<Self, Self::Error> {
        value.try_transmute::<OnGround>()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging()?;

    let client = SimConnect::new("Simple Program");

    match client {
        Ok(mut client) => loop {
            let notification = client.get_next_dispatch()?;

            match notification {
                Some(Notification::Open) => {
                    info!("Open");

                    client.register_object::<GpsData>()?;
                    client.register_object::<OnGround>()?;
                }
                Some(Notification::Data(data)) => {
                    if let Ok(gps_data) = GpsData::try_from(&data) {
                        info!("GPS Data: {gps_data:?}");
                        continue;
                    }
                    if let Ok(on_ground) = OnGround::try_from(&data) {
                        info!("On Ground data: {on_ground:?}");
                        continue;
                    }
                }
                _ => (),
            }
        },
        Err(e) => {
            error!("{e:?}")
        }
    }

    Ok(())
}
