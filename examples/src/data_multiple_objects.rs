use simconnect_sdk::{Notification, SimConnect, SimConnectObject};

/// A data structure that will be used to receive data from SimConnect.
/// See the documentation of `SimConnectObject` for more information on the arguments of the `simconnect` attribute.
#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second")]
#[allow(dead_code)]
struct GpsData {
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
    lat: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
    lon: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
    alt: f64,
}

/// A second data structure that will be used to receive data from SimConnect.
/// See the documentation of `SimConnectObject` for more information on the arguments of the `simconnect` attribute.
#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second", condition = "changed")]
#[allow(dead_code)]
pub struct OnGround {
    #[simconnect(name = "SIM ON GROUND", unit = "bool")]
    sim_on_ground: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SimConnect::new("Receiving data example");

    match client {
        Ok(mut client) => loop {
            let notification = client.get_next_dispatch()?;

            match notification {
                Some(Notification::Open) => {
                    println!("Connection opened.");

                    // After the connection is successfully open, we register the structs
                    client.register_object::<GpsData>()?;
                    client.register_object::<OnGround>()?;
                }
                Some(Notification::Object(data)) => {
                    if let Ok(gps_data) = GpsData::try_from(&data) {
                        println!("{gps_data:?}");
                        // We've already got our data, there's no point in trying another in this iteration
                        continue;
                    }
                    if let Ok(on_ground) = OnGround::try_from(&data) {
                        println!("{on_ground:?}");
                        // We've already got our data, there's no point in trying another in this iteration
                        continue;
                    }
                }
                _ => (),
            }

            // sleep for about a frame to reduce CPU usage
            std::thread::sleep(std::time::Duration::from_millis(16));
        },
        Err(e) => {
            println!("Error: {e:?}")
        }
    }

    Ok(())
}
