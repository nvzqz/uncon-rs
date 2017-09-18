# uncon [![Crates.io][crate-badge] ![Downloads][crate-dl]][crate] [![Build Status][travis-badge]][travis]

Traits for **un**checked **con**versions between types in Rust.

[Documentation][crate-doc]

## Changes

Separate changelogs are available for
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
uncon_derive = "1.0.3"
```

## Usage

This project allows for converting values between types with the _unsafe_
assumption that whatever required invariants are met.

For example, a `Value` instance must have a bit pattern from 0 to 2, inclusive.

```rust
#[macro_use]
extern crate uncon_derive;
extern crate uncon;

use uncon::*;

#[derive(FromUnchecked, PartialEq)]
#[repr(u8)]
enum Value {
    X, Y, Z
}

fn main() {
    let v = unsafe { Value::from_unchecked(2) };
    assert_eq!(v, Value::Z);

    // Undefined behavior:
    let u = unsafe { Value::from_unchecked(3) };
}
```

To allow for safe (but possibly slower) conversions, one may also implement
`From<T>` via `FromUnchecked<T>` where a mask or other operation is used to
make the input value valid:

```rust
impl From<u8> for Value {
    fn from(bits: u8) -> Value {
        unsafe { Value::from_unchecked(bits % 3) }
    }
}
```

[Some types](https://docs.rs/uncon/1.0.0/uncon/trait.FromUnchecked.html#implementors)
already implement `FromUnchecked` out-of-the-box.

### Defined Behavior

This project is not an excuse to go around and create chaos through
[undefined behavior][ub]. These operations should only ever be done when speed
is necessary and it is _**absolutely certain**_ that they will not cause strange
behavior.

Don't always reach for `mem::transmute`. There are usually
[alternatives][transmute-alt]. [Here's][transmute-nom] a good list of reasons
why it should be avoided.

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

[ub]:            https://en.wikipedia.org/wiki/Undefined_behavior
[transmute-alt]: https://doc.rust-lang.org/std/mem/fn.transmute.html#alternatives
[transmute-nom]: https://doc.rust-lang.org/nomicon/transmutes.html

[license-mit]:    https://github.com/nvzqz/uncon-rs/blob/master/LICENSE-MIT
[license-apache]: https://github.com/nvzqz/uncon-rs/blob/master/LICENSE-APACHE
