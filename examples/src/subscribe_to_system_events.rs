use simconnect_sdk::{Notification, SimConnect, SystemEvent, SystemEventRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SimConnect::new("System Events example");

    match client {
        Ok(mut client) => loop {
            let notification = client.get_next_dispatch()?;

            match notification {
                Some(Notification::Open) => {
                    println!("Connection opened.");

                    // After the connection is successfully open
                    // We request the system events we're interested in
                    client.subscribe_to_system_event(SystemEventRequest::FourSeconds)?;
                    client.subscribe_to_system_event(SystemEventRequest::AircraftLoaded)?;
                    client.subscribe_to_system_event(SystemEventRequest::Crashed)?;
                    client.subscribe_to_system_event(SystemEventRequest::FlightLoaded)?;
                    client.subscribe_to_system_event(SystemEventRequest::FlightPlanActivated)?;
                    client.subscribe_to_system_event(SystemEventRequest::FlightPlanDeactivated)?;
                    client.subscribe_to_system_event(SystemEventRequest::Pause)?;
                    client.subscribe_to_system_event(SystemEventRequest::Sim)?;
                    client.subscribe_to_system_event(SystemEventRequest::Sound)?;
                    client.subscribe_to_system_event(SystemEventRequest::View)?;
                }
                Some(Notification::SystemEvent(event)) => match event {
                    SystemEvent::FourSeconds => {
                        println!("FourSeconds ping received.");

                        // After we have receive one FourSeconds event notification, we unsubscribe from it
                        client.unsubscribe_from_system_event(SystemEventRequest::FourSeconds)?;
                        println!("FourSeconds subscription stopped.");
                    }
                    SystemEvent::AircraftLoaded { file_name } => {
                        println!("AircraftLoaded: {file_name}.");
                    }
                    SystemEvent::Crashed => {
                        println!("Crashed.");
                    }
                    SystemEvent::FlightLoaded { file_name } => {
                        println!("FlightLoaded: {file_name}.");
                    }
                    SystemEvent::FlightPlanActivated { file_name } => {
                        println!("FlightPlanActivated: {file_name}.");
                    }
                    SystemEvent::FlightPlanDeactivated => {
                        println!("FlightPlanDeactivated.");
                    }
                    SystemEvent::Pause { state } => {
                        println!("Pause: {state}.");
                    }
                    SystemEvent::Sim { state } => {
                        println!("Sim: {state}.");
                    }
                    SystemEvent::Sound { state } => {
                        println!("Sound: {state}.");
                    }
                    SystemEvent::View { view } => {
                        println!("View: {view:?}.");
                    }
                    _ => {}
                },
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
