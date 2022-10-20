/// This example shows all the work that the [`simconnect_sdk::SimConnectObject`] macro is doing behind the scenes.
/// You're probably better off using the macro in a real-life use-case.
use simconnect_sdk::{
    fixed_c_str_to_string, Condition, DataType, Notification, Object, Period, SimConnect,
    SimConnectError, SimConnectObjectExt,
};

/// A data structure that will be used to receive data from SimConnect.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AirplaneData {
    title: String,
    category: String,
    lat: f64,
    lon: f64,
    alt: f64,
    sim_on_ground: bool,
}

/// An intermediate data structure that will map 1:1 to the object received from SimConnect.
#[repr(C, packed)]
struct AirplaneDataCPacked {
    title: [i8; 256],
    category: [i8; 256],
    lat: f64,
    lon: f64,
    alt: f64,
    sim_on_ground: bool,
}

impl SimConnectObjectExt for AirplaneData {
    fn register(client: &mut SimConnect, id: u32) -> Result<(), SimConnectError> {
        client.add_to_data_definition(id, "TITLE", "", DataType::String)?;
        client.add_to_data_definition(id, "CATEGORY", "", DataType::String)?;
        client.add_to_data_definition(id, "PLANE LATITUDE", "degrees", DataType::Float64)?;
        client.add_to_data_definition(id, "PLANE LONGITUDE", "degrees", DataType::Float64)?;
        client.add_to_data_definition(id, "PLANE ALTITUDE", "feet", DataType::Float64)?;
        client.add_to_data_definition(id, "SIM ON GROUND", "", DataType::Float64)?;

        client.request_data_on_sim_object(id, Period::Second, Condition::None, 0)?;

        Ok(())
    }
}

impl TryFrom<&Object> for AirplaneData {
    type Error = SimConnectError;

    fn try_from(value: &Object) -> Result<Self, Self::Error> {
        let raw = value.try_transmute::<AirplaneData, AirplaneDataCPacked>()?;

        Ok(AirplaneData {
            title: fixed_c_str_to_string(&raw.title),
            category: fixed_c_str_to_string(&raw.category),
            lat: raw.lat,
            lon: raw.lon,
            alt: raw.alt,
            sim_on_ground: raw.sim_on_ground,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SimConnect::new("Receiving data example");

    match client {
        Ok(mut client) => {
            let mut notifications_received = 0;

            loop {
                let notification = client.get_next_dispatch()?;

                match notification {
                    Some(Notification::Open) => {
                        println!("Connection opened.");

                        // After the connection is successfully open, we register the struct
                        client.register_object::<AirplaneData>()?;
                    }
                    Some(Notification::Object(data)) => {
                        if let Ok(gps_data) = AirplaneData::try_from(&data) {
                            println!("{gps_data:?}");

                            notifications_received += 1;

                            // After we have received 10 notifications, we unregister the struct
                            if notifications_received > 10 {
                                client.unregister_object::<AirplaneData>()?;
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
