# URM: Unified Repository Manager

URM is a reinvented wheel for simple repository management.

## Design

The "Unified" in the name "URM" means that the user interfaces (UIs) and the application programming interfaces (APIs) are combined together, and can be accessed at the same location (URL). The server inspects the `Accept` HTTP header sent by HTTP clients, and send responses in differnet formats (`Content-Type`s) according to it:

- If `Accept: text/html`, then `Content-Type: text/html`. Typically, browsers automatically send this `Accept` header.
- If `Accept: application/json`, then `Content-Type: application/json`.
  - So far, if the `Accept` header is not specified, or it's `Accept: */*` (this is typically sent by cURL), then JSON is also returned, making such behavior the default.

This is perhaps useless, but whatever.

## Build

This project depends on [Rocket][rocket], which is a powerful but simple web framework for Rust. Since [Rocket][rocket] requires [Rust nightly][rust-nightly], a nightly version of the Rust compiler is required to compile this project. If you don't already have the Rust compiler installed, use [Rustup][rustup].

After you have the nightly Rust toolchain set up, invoke `cargo build` to build this project in debug mode, or `cargo run` to directly run the project compiled in debug mode. Release mode can be toggled with the option `--release`.

```shell
cargo build             # Build the project in debug mode
# cargo build --release # Build the project in release mode

cargo run               # Run the project compiled in debug mode
# cargo run --release   # Run the project compiled in release mode
```

[rocket]: https://rocket.rs/
[rust-nightly]: https://doc.rust-lang.org/book/appendix-07-nightly-rust.html
[rustup]: https://rustup.rs/

## Configure

URM will read [`urm.toml`](urm.toml) in the running directory as the configuration file. There is an example configuration file in the root of source tree; read it and know what can be configured.

## License

This software is licensed under the GNU General Public License v3.0. See [COPYING](COPYING) for details.
