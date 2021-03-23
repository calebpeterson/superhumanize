# superhumanize

Make `friendly_machine_produced[labels]` into `Friendly Machine Produced Labels`.

## Implementation Notes

`superhumanize` is implemented in Rust as a proof-of-concept to explore whether or not Rust + WASM is production-ready.

## Building

```
> wasm-pack build --target nodejs
```

## Testing

### Rust-based Unit Tests

```
> cargo test
```

### Node.js-based Integration Tests

```
# Ensure that the wasm package is built
> wasm-pack build --target nodejs

# Run the integration test suite
> node tests/integration-tests
```
