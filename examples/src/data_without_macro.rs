#![allow(dead_code)]

use simconnect_sdk::{
    Condition, DataType, Notification, Object, Period, SimConnect, SimConnectError,
    SimConnectObjectExt,
};

/// A data structure that will be used to receive data from SimConnect.
#[derive(Debug, Clone)]
pub struct GpsData {
    lat: f64,
    lon: f64,
    alt: f64,
}

impl SimConnectObjectExt for GpsData {
    fn register(client: &mut SimConnect, id: u32) -> Result<(), SimConnectError> {
        client.add_to_data_definition(id, "PLANE LATITUDE", "degrees", DataType::Float64)?;
        client.add_to_data_definition(id, "PLANE LONGITUDE", "degrees", DataType::Float64)?;
        client.add_to_data_definition(id, "PLANE ALTITUDE", "meters", DataType::Float64)?;

        client.request_data_on_sim_object(id, Period::Second, Condition::None, 0)?;

        Ok(())
    }
}

impl TryFrom<&Object> for GpsData {
    type Error = SimConnectError;

    fn try_from(value: &Object) -> Result<Self, Self::Error> {
        value.try_transmute::<GpsData>()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SimConnect::new("Simple Program");

    match client {
        Ok(mut client) => {
            let mut notifications_received = 0;

            loop {
                let notification = client.get_next_dispatch()?;

                match notification {
                    Some(Notification::Open) => {
                        println!("Connection opened.");

                        // After the connection is successfully open, we register the struct
                        client.register_object::<GpsData>()?;
                    }
                    Some(Notification::Object(data)) => {
                        if let Ok(gps_data) = GpsData::try_from(&data) {
                            println!("{gps_data:?}");

                            notifications_received += 1;

                            // After we have received 10 notifications, we unregister the struct
                            if notifications_received > 10 {
                                client.unregister_object::<GpsData>()?;
                                println!("Subscription stopped.");
                                break;
                            }
                        }
                    }
                    _ => (),
                }

                // sleep for about a frame to reduce CPU usage
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
        }
        Err(e) => {
            println!("Error: {e:?}")
        }
    }

    Ok(())
}
