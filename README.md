# Superball! Bounce Server

Superball! is a simple utility to aid in testing of callback-based web services.
It exposes these endpoints:

1. GET `/` A simple message from the server
1. GET `/echo/<anything>` Echo back what you send in the url
1. GET `/bounce?url=<url-encoded-url>` Redirect to the specified url
1. GET `/session` Show the default session

## Contributing

Superball! can now be built using two different methods: using local Rust toolchains or using Docker.
It can be deployed using Docker to containerize the compiled application.
If you'd like to contribute to Superball! then please follow these steps:

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

## Building with Docker

The Docker method to build uses Fletcher Nichol's excellent [Docker containers for the Rust toolchain](docker-rust).

1. Get [Docker](http://www.docker.com/)
1. Open a terminal and `cd` to the Superball! root directory (where this README is located)
1. Create a container to act as a cargo data cache (so you don't have to download packages everytime you build!)
    ```sh
    docker create -v /cargo --name cargo-cache tianon/true /bin/true
    ```
1. To run tests or debug the code, you can use the container toolchain to run Superball! directly
    ```sh
    docker run --rm -v $(pwd):/src --volumes-from cargo-cache fnichol/rust:nightly cargo run
    ```
1. When you're ready to finalize and build, use the container to build Superball! (writes into your `target/release` directory)
    ```sh
    docker run --rm -v $(pwd):/src --volumes-from cargo-cache fnichol/rust:nightly cargo build --release
    ```
1. Build the Superball! container
    ```sh
    docker build --tag='superball:vX.Y.Z' .
    ```
1. Edit `docker-compose.yml` to point to the correct version of Superball! that you just built
1. Run Superball! from its container!
    ```sh
    docker-compose up
    ```

## Building and Running with Local Toolchain

### Requirements

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

[rustup]: https://www.rustup.rs/
[docker-rust]: https://github.com/fnichol/docker-rust