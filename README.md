# SimConnect SDK in Rust

## Usage

```toml
[dependencies]
simconnect-sdk = { version = "0.1", features = ["derive"] }
```

```rust
use simconnect_sdk::{Notification, SimConnect, SimConnectObject};

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
    let client = SimConnect::new("Simple Program");

    match client {
        Ok(mut client) => loop {
            let notification = client.get_next_dispatch()?;

            match notification {
                Some(Notification::Open) => {
                    println!("Open");

                    // The struct must be registered after the connection is successfully open
                    client.register_object::<GpsData>()?;
                }
                Some(Notification::Data(data)) => {
                    if let Ok(gps_data) = GpsData::try_from(&data) {
                        println!("GPS Data: {gps_data:?}");
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
```

See [more examples](https://github.com/mihai-dinculescu/simconnect-sdk/tree/main/examples).
