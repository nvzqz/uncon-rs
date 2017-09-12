# uncon [![Crates.io][crate-badge] ![Downloads][crate-dl]][crate] [![Build Status][travis-badge]][travis]

Traits for **un**checked **con**versions between types in Rust.

[Documentation][crate-doc]

## Changes

Separate changlogs are available for
[`uncon`](https://github.com/nvzqz/uncon-rs/blob/master/CHANGELOG.md) and
[`uncon_derive`](https://github.com/nvzqz/uncon-rs/blob/master/derive/CHANGELOG.md).

Although they may differ in version number, `uncon_derive` is always compatible
with current major version of `uncon`.

## Installation

This crate is available [on crates.io][crate] and can be used by adding the
following to your project's `Cargo.toml`:

```toml
[dependencies]
uncon = "1.0.0"

# Derive:
uncon_derive = "1.0.2"
```

and this to your crate root:

```rust
extern crate uncon;

// Derive:
#[macro_use]
extern crate uncon_derive
```

## License

This project is released under either:

- [MIT License][license-mit]

- [Apache License (Version 2.0)][license-apache]

at your choosing.

[crate]:       https://crates.io/crates/uncon
[crate-dl]:    https://img.shields.io/crates/d/uncon.svg
[crate-doc]:   https://docs.rs/uncon/
[crate-badge]: https://img.shields.io/crates/v/uncon.svg

[travis]:       https://travis-ci.org/nvzqz/uncon-rs
[travis-badge]: https://travis-ci.org/nvzqz/uncon-rs.svg?branch=master

[license-mit]:    https://github.com/nvzqz/uncon-rs/blob/master/LICENSE-MIT
[license-apache]: https://github.com/nvzqz/uncon-rs/blob/master/LICENSE-APACHE
