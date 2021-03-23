# superhumanize

Make `friendly_machine_produced[labels]` into `Friendly Machine Produced Labels`.

## Implementation Notes

`superhumanize` is implemented in Rust as a proof-of-concept to explore whether or not Rust + WASM is production-ready.

## Prerequisites

It is assumed that you have the [`cargo-make` crate](https://github.com/sagiegurari/cargo-make#installation) installed so that `cargo make ...` commands will work.

## Building

```
> cargo make build
```

## Testing

### Rust-based Unit Tests

```
> cargo make test
```

### Node.js-based Integration Tests

```
> cargo make npm-test
```

## Publishing

### To NPM

```
> cargo make npm-publish
```
