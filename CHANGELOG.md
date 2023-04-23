# Change Log

All notable changes to this project will be documented in this
file. This change log follows the conventions of
[keepachangelog.com][keepachangelog].

## [Unreleased]

### Added

- `SimConnect_TransmitClientEvent` has been implemented.  Use `SimConnect::register_event` to set up the client event(s) prior to using the `SimConnect_TransmitClientEvent` call.
- Added a few NAV/COM and Transponder related client events.

## [v0.2.2] - 2023-02-22

### Changed
- Updated to MSFS SDK v0.20.5.0.
- `SimConnect::get_next_dispatch` now takes a `&mut self` in order to be able to clean up requests that have returned all the results they ever will.

### Fixed
- `SimConnect::request_facilities_list` calls now automatically clean up the request after all the data is received.

## [v0.2.1] - 2022-10-29

### Added

- `#[derive(PartialEq)]` has been added to `Condition`, `DataType`, `FacilityType`, `NotificationGroup`, `Period`, `ViewType`, `ClientEvent`, `SystemEventRequest` and `SystemEvent`.
- `#[derive(Eq)]` has been added to `Condition`, `DataType`, `FacilityType`, `NotificationGroup`, `Period`, `ViewType`, `ClientEvent` and `SystemEventRequest`.

### Changed

- The GitHub repository has been renamed from `mihai-dinculescu/simconnect-sdk` to `mihai-dinculescu/simconnect-sdk-rs`.

## [v0.2.0] - 2022-10-29

### Added

- `Notification::SystemEvent`, `SystemEventRequest` and `SystemEvent` have been added. System Events can be subscribed to by using `SimConnect::subscribe_to_system_event` and unsubscribed from by using `SimConnect::unsubscribe_from_system_event`.

### Changed

- `Notification::Event` has been renamed to `Notification::ClientEvent`.
- `Event` has been renamed to `ClientEvent` and marked as `non_exhaustive`.

## [v0.1.3] - 2022-10-24

### Changed

- `SimConnect::get_next_dispatch` will now return an error of type `SimConnectError::UnimplementedMessageType` instead of panicking on unrecognized notification types.
- `SimConnect::get_next_dispatch` will now return an error of type `SimConnectError::SimConnectException` instead of `Notification::Exception`.
- `SimConnectError::SimConnectUnrecognizedEvent` has been renamed to `SimConnectError::UnimplementedEventType`.
- `#[non_exhaustive]` has been added to the `SimConnectError` and `Notification` enums.
- The tracing information has been adjusted to be at the `info` and `debug` levels instead of `info`.

## [v0.1.2] - 2022-10-22

### Added

- `Condition`, `DataType`, `FacilityType` and `Period` now derive `Debug`.

### Fixed

- Docs.rs build should now pass. Take 2.

## [v0.1.1] - 2022-10-21

### Changed

- The docs and README files have been improved.

### Fixed

- Docs.rs build should now pass.

## [v0.1.0] - 2022-10-20

### Initial Release of simconnect-sdk

[unreleased]: https://github.com/mihai-dinculescu/simconnect-sdk-rs
[v0.2.2]: https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/v0.2.2
[v0.2.1]: https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/v0.2.1
[v0.2.0]: https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/v0.2.0
[v0.1.3]: https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/v0.1.3
[v0.1.2]: https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/v0.1.2
[v0.1.1]: https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/v0.1.1
[v0.1.0]: https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/v0.1.0
[keepachangelog]: https://keepachangelog.com
