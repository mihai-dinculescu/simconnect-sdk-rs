use simconnect_sdk::{ClientEvent, ClientEventRequest, Notification, SimConnect};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SimConnect::new("Client Events example");

    let mut throttle_events_received = 0;
    let mut elevator_events_received = 0;

    match client {
        Ok(mut client) => loop {
            let notification = client.get_next_dispatch()?;

            match notification {
                Some(Notification::Open) => {
                    println!("Connection opened.");

                    // After the connection is successfully open
                    // We subscribe to the client events we're interested in
                    client.subscribe_to_client_event(ClientEventRequest::Throttle1Set)?;
                    client.subscribe_to_client_event(ClientEventRequest::AxisElevatorSet)?;
                }
                Some(Notification::ClientEvent(event)) => match event {
                    ClientEvent::Throttle1Set { value } => {
                        println!("Throttle1Set: {value}");

                        throttle_events_received += 1;
                        if throttle_events_received >= 9 {
                            // We unsubscribe from the client event after we receive 10 of them
                            // This might run multiple times if there are more events queued up
                            println!("Unsubscribing from Throttle1Set...");
                            client
                                .unsubscribe_from_client_event(ClientEventRequest::Throttle1Set)?;
                        }
                    }
                    ClientEvent::AxisElevatorSet { value } => {
                        println!("AxisElevatorSet: {value}");

                        elevator_events_received += 1;
                        if elevator_events_received >= 9 {
                            // We unsubscribe from the client event after we receive 10 of them
                            // This might run multiple times if there are more events queued up
                            println!("Unsubscribing from AxisElevatorSet...");
                            client.unsubscribe_from_client_event(
                                ClientEventRequest::AxisElevatorSet,
                            )?;
                        }
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
