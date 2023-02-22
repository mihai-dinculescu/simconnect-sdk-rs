use simconnect_sdk::{FacilityType, Notification, SimConnect};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SimConnect::new("Facilities example");

    match client {
        Ok(mut client) => loop {
            let notification = client.get_next_dispatch()?;

            match notification {
                Some(Notification::Open) => {
                    println!("Connection opened.");

                    // After the connection is successfully open

                    // We request the existing list of airports in the facilities cache
                    client.request_facilities_list(FacilityType::Airport)?;

                    // We subscribe to the current list and future additions of waypoints, NDBs and VORs
                    client.subscribe_to_facilities(FacilityType::Waypoint)?;
                    client.subscribe_to_facilities(FacilityType::NDB)?;
                    client.subscribe_to_facilities(FacilityType::VOR)?;
                }
                Some(Notification::AirportList(data)) => {
                    for record in data {
                        // The returned list is quite large, so we look for a particular record
                        if record.icao == "EGSC" {
                            println!("{record:?}");
                            // there's no need to unsubscribe
                            // because this is a one-off request, not a subscription
                        }
                    }
                }
                Some(Notification::WaypointList(data)) => {
                    for record in data {
                        // The returned list is quite large, so we look for a particular record
                        if record.icao == "BRAIN" {
                            println!("{record:?}");
                            // we've got the entry we're interesting in - we can unsubscribe now
                            client.unsubscribe_to_facilities(FacilityType::Waypoint)?;
                        }
                    }
                }
                Some(Notification::NdbList(data)) => {
                    for record in data {
                        // The returned list is quite large, so we look for a particular record
                        if record.icao == "CAM" {
                            println!("{record:?}");
                            // we've got the entry we're interesting in - we can unsubscribe now
                            client.unsubscribe_to_facilities(FacilityType::NDB)?;
                        }
                    }
                }
                Some(Notification::VorList(data)) => {
                    for record in data {
                        // The returned list is quite large, so we look for a particular record
                        if record.icao == "LON" {
                            println!("{record:?}");
                            // we've got the entry we're interesting in - we can unsubscribe now
                            client.unsubscribe_to_facilities(FacilityType::VOR)?;
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
