use simconnect_sdk::{Notification, SimConnect, SimConnectObject};
use tracing::{error, info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// A data structure that will be used to receive data from SimConnect.
/// See the documentation of `SimConnectObject` for more information on the arguments of the `simconnect` attribute.
#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second")]
#[allow(dead_code)]
struct AirplaneData {
    #[simconnect(name = "TITLE")]
    title: String,
    #[simconnect(name = "CATEGORY")]
    category: String,
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
    lat: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
    lon: f64,
    #[simconnect(name = "PLANE ALTITUDE", unit = "feet")]
    alt: f64,
    #[simconnect(name = "SIM ON GROUND")]
    sim_on_ground: bool,
}

/// To see all tracing information at the `trace` level and above (most verbose), run:
///
/// ```bash
/// RUST_LOG=trace cargo run --bin data_with_tracing
/// ```
///
/// To see all tracing information at the `debug` level and above (less verbose than `info`), run:
///
/// ```bash
/// RUST_LOG=debug cargo run --bin data_with_tracing
/// ```
fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging()?;

    let client = SimConnect::new("Receiving data example");

    match client {
        Ok(mut client) => {
            let mut notifications_received = 0;

            loop {
                let notification = client.get_next_dispatch()?;

                match notification {
                    Some(Notification::Open) => {
                        info!("Open");

                        // After the connection is successfully open, we register the struct
                        client.register_object::<AirplaneData>()?;
                    }
                    Some(Notification::Object(data)) => {
                        if let Ok(airplane_data) = AirplaneData::try_from(&data) {
                            info!("{airplane_data:?}");

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
            error!("{e:?}")
        }
    }

    Ok(())
}

fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;
    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_span_events(fmt::format::FmtSpan::FULL);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    Ok(())
}
