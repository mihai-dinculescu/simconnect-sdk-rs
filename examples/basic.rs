use logging::setup_logging;
use simconnect_sdk::{
    ConditionEnum, DataType, Notification, NotificationData, PeriodEnum, SimConnect,
    SimConnectError,
};
use tracing::{error, info};

mod logging;

#[derive(Debug, Clone)]
pub struct GpsData {
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub gps_ground_magnetic_track: f64,
    pub gps_ground_speed: f64,
}

impl simconnect_sdk::SimConnectObject for GpsData {
    fn register(client: &mut SimConnect, id: u32) -> Result<(), SimConnectError> {
        client.add_to_data_definition(id, "PLANE LATITUDE", "degrees", DataType::F64)?;
        client.add_to_data_definition(id, "PLANE LONGITUDE", "degrees", DataType::F64)?;
        client.add_to_data_definition(id, "PLANE ALTITUDE", "meters", DataType::F64)?;
        client.add_to_data_definition(id, "GPS GROUND MAGNETIC TRACK", "degrees", DataType::F64)?;
        client.add_to_data_definition(
            id,
            "GPS GROUND SPEED",
            "Meters per second",
            DataType::F64,
        )?;
        client.request_data_on_sim_object(id, PeriodEnum::Second, ConditionEnum::None)?;

        Ok(())
    }
}

impl TryFrom<&NotificationData> for GpsData {
    type Error = ();

    fn try_from(value: &NotificationData) -> Result<Self, Self::Error> {
        value.try_into::<GpsData>().ok_or(())
    }
}

#[derive(Debug, Clone)]
pub struct OnGround {
    pub sim_on_ground: bool,
}

impl simconnect_sdk::SimConnectObject for OnGround {
    fn register(client: &mut SimConnect, id: u32) -> Result<(), SimConnectError> {
        client.add_to_data_definition(id, "SIM ON GROUND", "bool", DataType::Bool)?;
        client.request_data_on_sim_object(id, PeriodEnum::Second, ConditionEnum::None)?;

        Ok(())
    }
}

impl TryFrom<&NotificationData> for OnGround {
    type Error = ();

    fn try_from(value: &NotificationData) -> Result<Self, Self::Error> {
        value.try_into::<OnGround>().ok_or(())
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
                        info!("GPS Data: {:?}", gps_data);
                        continue;
                    }
                    if let Ok(on_ground) = OnGround::try_from(&data) {
                        info!("On Ground data: {:?}", on_ground);
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
