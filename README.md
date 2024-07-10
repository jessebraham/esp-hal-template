# esp-hal-template

One day, maybe, a collection of templates for creating applications using [esp-hal].

Currently, a single opinionated template for creating applications using [esp-hal] and [embassy]. Compatible with all devices supported by [esp-hal] (i.e. the entire ESP32 line of chips).

[esp-hal]: https://github.com/esp-rs/esp-hal
[embassy]: https://github.com/embassy-rs/embassy

# Quickstart

It is assumed that Rust, and in turn `cargo`, are already installed on your system. If you have not yet installed Rust, please visit <https://rustup.rs/> for installation instructions.

In order to use this template, you must first install [cargo-generate] and [probe-rs]:

```
cargo install cargo-generate probe-rs-tools
```

[cargo-generate]: https://github.com/cargo-generate/cargo-generate/
[probe-rs]: https://github.com/probe-rs/probe-rs/

With the prerequisite tools installed, you can generate a project from the template:

```
cargo generate jessebraham/esp-hal-template embassy
```

After answering the prompts, you can change into your project's directory and execute your application on-device using `probe-rs` with:

```
cargo run --release
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
