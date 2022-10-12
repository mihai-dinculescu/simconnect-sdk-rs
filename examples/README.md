# SimConnect SDK Examples

## Checkout the repository

```bash
git clone git@github.com:mihai-dinculescu/simconnect-sdk.git
cd simconnect-sdk
```

## Receiving data

```bash
cargo run --bin basic
```

## Using tracing

```bash
RUST_LOG=info cargo run --bin basic_with_tracing
```

## Receiving data without the derive macro

```bash
cargo run --bin basic_without_macro
```

## Multiple objects

```bash
cargo run --bin multiple_objects
```
