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
    #[simconnect(name = "PLANE ALTITUDE", unit = "feet")]
    alt: f64,
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
