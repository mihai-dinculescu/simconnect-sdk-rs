# SimConnect SDK Examples

## Checkout the repository

```bash
git clone git@github.com:mihai-dinculescu/simconnect-sdk.git
cd simconnect-sdk
```

## Receiving data

```bash
cargo run --bin data
```

## Receiving data with tracing

To see all tracing information at the `trace` level and above (most verbose), run:

```bash
RUST_LOG=trace cargo run --bin data_with_tracing
```

To see all tracing information at the `debug` level and above (less verbose than `info`), run:

```bash
RUST_LOG=debug cargo run --bin data_with_tracing
```

## Receiving data without the derive macro

```bash
cargo run --bin data_without_macro
```

## Receiving data using multiple objects

```bash
cargo run --bin data_multiple_objects
```

## Receiving facilities from cache

```bash
cargo run --bin facilities
```

## Receiving system events

```bash
cargo run --bin system_events
```
