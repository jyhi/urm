# URM: Unified Repository Manager

URM is a reinvented wheel for _simple_ repository / inventory management. Originally designed for asset management in small laboratories, URM provides a light-weight and straightforward way to manage things without the burden put by enterprise-class inventory management systems.

## Design

The "Unified" in the name "URM" means that the user interfaces (UIs) and the application programming interfaces (APIs) are combined together, and can be accessed at the same location (URL). The presented web page is simply a kind of presentation of data which is human-readable, while the APIs present data which is machine-readable.

Technically, this is simple to achieve. The server inspects the `Accept` HTTP header sent by HTTP clients, and send responses in differnet formats (`Content-Type`s) according to it:

- If `Accept: text/html`, then `Content-Type: text/html`. Typically, browsers automatically send this `Accept` header.
- If `Accept: application/json`, then `Content-Type: application/json`.
  - This is also the default behavior: if the `Accept` header is not specified, or it's `Accept: */*` (this is typically sent by cURL), then JSON is also returned.

This is perhaps useless, but whatever.

## Use

The tarball containing the binary release consists of a single directory called `urm/`, where the main executable of URM named `urm` is located. There is another executable named `genuser`, which is used for generating a user entry ready for inserting into the corresponding MongoDB collection (table).

Before running URM, you may want to adjust some settings to suit your environment:

- To configure the serving behavior of URM, `Rocket.toml` needs to be tweaked. Consult [this documentation][rocket-doc] for how to configure the server, e.g. the listening address and port.
- To configure URM _per se_, edit [`urm.toml`](urm.toml); URM reads `urm.toml` in its running directory. The provided `urm.toml` is an example configuration file with comments, where all options have defaults and are optional. Read it and know what can be configured!

MongoDB should already be running. After the MongoDB daemon (`mongod`) starts, you need to use the provided `genuser` program to generate users that are permitted to edit the database:

```shell
# Usage: ./genuser <username> <password>
$ ./genuser urm mru
{"username":"urm","password":"...a long long string..."}
```

Assuming that you do not edit the options in the `[collection]` table in `urm.toml`, the output should be directly inserted into the `users` collection:

```shell
> use urm
switched to db urm
> db.users.insert({"username":"urm","password":"...a long long string..."})
WriteResult({ "nInserted" : 1 })
```

Finally, launch URM by simply invoking `./urm`.

## Build

This project depends on [Rocket][rocket], which is a powerful but simple web framework for Rust. Since [Rocket][rocket] requires [Rust nightly][rust-nightly], a nightly version of the Rust compiler is required to compile this project. If you don't already have the Rust compiler installed, use [Rustup][rustup].

After you have the nightly Rust toolchain set up, invoke `cargo build` to build this project in debug mode, or `cargo run` to directly run the project compiled in debug mode. Release mode can be toggled with the option `--release`.

```shell
cargo build             # Build the project in debug mode
# cargo build --release # Build the project in release mode

cargo run               # Run the project compiled in debug mode
# cargo run --release   # Run the project compiled in release mode
```

A full list of dependencies that URM is relying on can be viewed at [Cargo.toml](Cargo.toml).

[rocket]: https://rocket.rs/
[rust-nightly]: https://doc.rust-lang.org/book/appendix-07-nightly-rust.html
[rustup]: https://rustup.rs/

## License

This software is licensed under the GNU General Public License v3.0. See [COPYING](COPYING) for details.
