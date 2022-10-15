use simconnect_sdk::{FacilityType, Notification, SimConnect};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SimConnect::new("Simple Program");

    match client {
        Ok(mut client) => loop {
            let notification = client.get_next_dispatch()?;

            match notification {
                Some(Notification::Open) => {
                    println!("Open");

                    // After the connection is successfully open, we subscribe to all facility types that we are interested in
                    client.subscribe_to_facilities(FacilityType::Airport)?;
                    client.subscribe_to_facilities(FacilityType::Waypoint)?;
                    client.subscribe_to_facilities(FacilityType::NDB)?;
                    client.subscribe_to_facilities(FacilityType::VOR)?;
                }
                Some(Notification::AirportList(data)) => {
                    for record in data {
                        // The returned list is quite large, so we look for a particular record
                        if record.icao == "EGSC" {
                            println!("{record:?}");
                        }
                    }
                }
                Some(Notification::WaypointList(data)) => {
                    for record in data {
                        // The returned list is quite large, so we look for a particular record
                        if record.icao == "BRAIN" {
                            println!("{record:?}");
                        }
                    }
                }
                Some(Notification::NdbList(data)) => {
                    for record in data {
                        // The returned list is quite large, so we look for a particular record
                        if record.icao == "CAM" {
                            println!("{record:?}");
                        }
                    }
                }
                Some(Notification::VorList(data)) => {
                    for record in data {
                        // The returned list is quite large, so we look for a particular record
                        if record.icao == "LON" {
                            println!("{record:?}");
                        }
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
