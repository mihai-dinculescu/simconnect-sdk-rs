# SimConnect SDK in Rust

[![Crates][crates_badge]][crates]
[![Documentation][documentation_badge]][documentation]
[![CI][ci_badge]][ci]
[![license][license_badge]][license]
[![Crates.io][crates_downloads_badge]][crates]\
An opinionated SimConnect SDK that encapsulates the C API fully and optimizes for developer experience.

## Usage

```toml
[dependencies]
simconnect-sdk = { version = "0.1", features = ["derive"] }
```

```rust
use simconnect_sdk::{Notification, SimConnect, SimConnectObject};

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
                        if let Ok(airplane_data) = AirplaneData::try_from(&data) {
                            println!("{airplane_data:?}");

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
```

See [more examples][examples].

## Contributing

Contributions are welcome and encouraged! See [/CONTRIBUTING.md][contributing].

## Feature table

### General

| Feature                                 | Status  | Comment     |
| --------------------------------------- | ------- | ----------- |
| DispatchProc                            |         |             |
| SimConnect_Open                         | &check; |             |
| SimConnect_Close                        | &check; |             |
| SimConnect_CallDispatch                 |         |             |
| SimConnect_GetNextDispatch              | &check; |             |
| SimConnect_RequestSystemState           |         |             |
| SimConnect_MapClientEventToSimEvent     | -       | Coming soon |
| SimConnect_SubscribeToSystemEvent       |         |             |
| SimConnect_SetSystemEventState          |         |             |
| SimConnect_UnsubscribeFromSystemEvent   |         |             |
| SimConnect_SetNotificationGroupPriority | -       | Coming soon |

### Events And Data

| Feature                                      | Status  | Comment                             |
| -------------------------------------------- | ------- | ----------------------------------- |
| SimConnect_RequestDataOnSimObject            | &check; | Only for SIMCONNECT_OBJECT_ID_USER  |
| SimConnect_RequestDataOnSimObjectType        | -       | Coming soon                         |
| SimConnect_AddClientEventToNotificationGroup | -       | Coming soon                         |
| SimConnect_RemoveClientEvent                 |         |                                     |
| SimConnect_TransmitClientEvent               |         |                                     |
| SimConnect_TransmitClientEvent_EX1           |         |                                     |
| SimConnect_MapClientDataNameToID             |         |                                     |
| SimConnect_RequestClientData                 |         |                                     |
| SimConnect_CreateClientData                  |         |                                     |
| SimConnect_AddToClientDataDefinition         |         |                                     |
| SimConnect_AddToDataDefinition               | &check; | Supports `f64`, `bool` and `String` |
| SimConnect_SetClientData                     |         |                                     |
| SimConnect_SetDataOnSimObject                |         |                                     |
| SimConnect_ClearClientDataDefinition         |         |                                     |
| SimConnect_ClearDataDefinition               | &check; |                                     |
| SimConnect_MapInputEventToClientEvent        |         |                                     |
| SimConnect_RequestNotificationGroup          |         |                                     |
| SimConnect_ClearInputGroup                   |         |                                     |
| SimConnect_ClearNotificationGroup            |         |                                     |
| SimConnect_RequestReservedKey                |         |                                     |
| SimConnect_SetInputGroupPriority             |         |                                     |
| SimConnect_SetInputGroupState                |         |                                     |
| SimConnect_RemoveInputEvent                  |         |                                     |

### AI Objects

| Feature                               | Status | Comment |
| ------------------------------------- | ------ | ------- |
| SimConnect_AICreateEnrouteATCAircraft |        |         |
| SimConnect_AICreateNonATCAircraft     |        |         |
| SimConnect_AICreateParkedATCAircraft  |        |         |
| SimConnect_AICreateSimulatedObject    |        |         |
| SimConnect_AIReleaseControl           |        |         |
| SimConnect_AIRemoveObject             |        |         |
| SimConnect_AISetAircraftFlightPlan    |        |         |

### Flights

| Feature                   | Status | Comment |
| ------------------------- | ------ | ------- |
| SimConnect_FlightLoad     |        |         |
| SimConnect_FlightSave     |        |         |
| SimConnect_FlightPlanLoad |        |         |

### Debug

| Feature                         | Status | Comment |
| ------------------------------- | ------ | ------- |
| SimConnect_GetLastSentPacketID  |        |         |
| SimConnect_RequestResponseTimes |        |         |
| SimConnect_InsertString         |        |         |
| SimConnect_RetrieveString       |        |         |

### Facilities

| Feature                                | Status  | Comment |
| -------------------------------------- | ------- | ------- |
| SimConnect_AddToFacilityDefinition     |         |         |
| SimConnect_RequestFacilitesList        | &check; |         |
| SimConnect_RequestFacilitiesList_EX1   |         |         |
| SimConnect_RequestFacilityData         |         |         |
| SimConnect_SubscribeToFacilities       | &check; |         |
| SimConnect_SubscribeToFacilities_EX1   |         |         |
| SimConnect_UnsubscribeToFacilities     | &check; |         |
| SimConnect_UnsubscribeToFacilities_EX1 |         |         |

### Missions

| Feature                                | Status | Comment |
| -------------------------------------- | ------ | ------- |
| SimConnect_CompleteCustomMissionAction |        |         |
| SimConnect_ExecuteMissionAction        |        |         |

## Credits

Inspired by [Sequal32/simconnect-rust][inspired_by].

[crates_badge]: https://img.shields.io/crates/v/simconnect-sdk.svg
[crates]: https://crates.io/crates/simconnect-sdk
[documentation_badge]: https://docs.rs/simconnect-sdk/badge.svg
[documentation]: https://docs.rs/simconnect-sdk
[ci_badge]: https://github.com/mihai-dinculescu/simconnect-sdk/workflows/CI/badge.svg?branch=main
[ci]: https://github.com/mihai-dinculescu/simconnect-sdk/actions
[license_badge]: https://img.shields.io/crates/l/simconnect-sdk.svg
[license]: https://github.com/mihai-dinculescu/simconnect-sdk/blob/main/LICENSE
[crates_downloads_badge]: https://img.shields.io/crates/d/simconnect-sdk?label=downloads
[examples]: https://github.com/mihai-dinculescu/simconnect-sdk/tree/main/examples
[contributing]: https://github.com/mihai-dinculescu/simconnect-sdk/blob/main/CONTRIBUTING.md
[inspired_by]: https://github.com/Sequal32/simconnect-rust
