# Superball! Bounce Server

Superball! is a simple utility to aid in testing of callback-based web services.
It exposes 3 endpoints:

1. `/` A simple message from the server
1. `/echo/<anything>` Echo back what you send in the url
1. `/bounce?url=<url-encoded-url>` Redirect to the specified url

## Requirements

- Rust (Instructions coming soon! Hint: [rustup])

## Building and Running

1. `cargo run`

[rustup]: https://www.rustup.rs/