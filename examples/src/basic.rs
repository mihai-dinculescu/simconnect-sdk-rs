#![allow(dead_code)]

use simconnect_sdk::{Notification, SimConnect, SimConnectObject};
use simconnect_sdk_examples::setup_logging;
use tracing::{error, info};

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second")]
struct GpsData {
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
    lat: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
    lon: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
    alt: f64,
    #[simconnect(name = "GPS GROUND MAGNETIC TRACK", unit = "degrees")]
    gps_ground_magnetic_track: f64,
    #[simconnect(name = "GPS GROUND SPEED", unit = "Meters per second")]
    gps_ground_speed: f64,
}

#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "changed")]
pub struct OnGround {
    #[simconnect(name = "SIM ON GROUND", unit = "bool")]
    sim_on_ground: bool,
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
