# Change Log

All notable changes to this project will be documented in this
file. This change log follows the conventions of
[keepachangelog.com][keepachangelog].

## [Unreleased]

## [v0.2.3] - 2026-02-22

### Added

- Client events are now implemented through `SimConnect::subscribe_to_client_event`, `SimConnect::unsubscribe_from_client_event` and `SimConnect::unsubscribe_from_all_client_events`.
- `subscribe_to_client_events.rs` example has been added.
- `SimConnectError::EventAlreadySubscribedTo` and `SimConnectError::EventNotSubscribedTo` error variants have been added.

### Changed

- A second call to `SimConnect::subscribe_to_system_event` for the same event will now return an error of type `SimConnectError::EventAlreadySubscribedTo` instead of `SimConnectError::SimConnectException`.
- The call to `SimConnect::unsubscribe_from_system_event` is now a NOOP when the system event is not subscribed to.
- `SimConnectError::UnimplementedMessageType` has been renamed to `SimConnectError::UnimplementedNotification`.

### Removed

- `SimConnect::register_event` has been replaced by the new client event functions.
- `NotificationGroup` has been removed in favor of an internally managed notification group.

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
[v0.2.3]: https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/v0.2.3
[v0.2.2]: https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/v0.2.2
[v0.2.1]: https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/v0.2.1
[v0.2.0]: https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/v0.2.0
[v0.1.3]: https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/v0.1.3
[v0.1.2]: https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/v0.1.2
[v0.1.1]: https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/v0.1.1
[v0.1.0]: https://github.com/mihai-dinculescu/simconnect-sdk-rs/tree/v0.1.0
[keepachangelog]: https://keepachangelog.com
