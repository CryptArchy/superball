# Superball! Bounce Server

Superball! is a simple utility to aid in testing of callback-based web services.
It exposes 3 endpoints:

1. `/` A simple message from the server
1. `/echo/<anything>` Echo back what you send in the url
1. `/bounce?url=<url-encoded-url>` Redirect to the specified url

## Requirements

- Rust
  1. Download and install [rustup]
  1. Follow instructions to set up the `stable` version of rust
    + It should also build using the nightly rust
    + Try it out with
    ```sh
    rustup toolchain install nightly
    rustup run nightly cargo build
    ```
  1. Verify your installations
    + Verson manager: `rustup -V`
    + Package manager: `cargo -V`
    + Compiler: `rustc -V`
- Rust Formatter: `rustfmt`
  1. `cargo install rustfmt`
  1. `cargo fmt`

## Building and Running

To build do:

```sh
cargo build
```

To build and run do:

```sh
cargo run
```

To run unit tests do:

```sh
cargo test
```

## Contributing

1. Fork the repo
1. Code the things
1. Log the things
1. Writes tests for the things
1. Run `cargo fmt` to make it pretty
1. Run `cargo test` and pass all the tests
1. Push it. Push it real good.
1. Submit a PR
1. ?????
1. Contribution!

[rustup]: https://www.rustup.rs/