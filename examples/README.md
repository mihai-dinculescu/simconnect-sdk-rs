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

## Receiving data using tracing

```bash
RUST_LOG=info cargo run --bin data_with_tracing
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
