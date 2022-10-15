#![allow(dead_code)]

use simconnect_sdk::{Notification, SimConnect, SimConnectObject};
use tracing::{error, info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// A data structure that will be used to receive data from SimConnect.
#[derive(Debug, Clone, SimConnectObject)]
#[simconnect(period = "second")]
struct GpsData {
    #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
    lat: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
    lon: f64,
    #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
    alt: f64,
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

                    // After the connection is successfully open, we register the struct
                    client.register_object::<GpsData>()?;
                }
                Some(Notification::Object(data)) => {
                    if let Ok(gps_data) = GpsData::try_from(&data) {
                        info!("{gps_data:?}");
                    }
                }
                _ => (),
            }

            // sleep for about a frame to reduce CPU usage
            std::thread::sleep(std::time::Duration::from_millis(16));
        },
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
