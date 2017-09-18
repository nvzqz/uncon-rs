# Changelog [![Crates.io][crate-badge]][crate]
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

**Note:** `uncon_derive` has [its own changelog][derive-log] separate from this one.

## [Unreleased]

## 1.1.0 - 2017-09-17
### Added
- Generic `FromUnchecked` implementations:

  | From                | Into          |
  | :------------------ | :------------ |
  | `&{,mut} [U]`       | `&{,mut} [T]` |
  | `Vec<U>`            | `Vec<T>`      |
  | `Box<U>`            | `Box<T>`      |
  | `Box<[U]>`          | `Box<[T]>`    |
  | `{Arc,Rc}<U>`       | `{Arc,Rc}<T>` |

## 1.0.0 - 2017-09-11
### Added
- `FromUnchecked` and `IntoUnchecked` traits.
- `#[no_std]` support
- `IntoUnchecked<U>` generic implementation for `T` where `U: FromUnchecked<T>`
- Generic `FromUnchecked` implementations:

  | From                | Into          |
  | :------------------ | :------------ |
  | `&{,mut} U`         | `&{,mut} T`   |
  | `*{const,mut} T`    | `&{,mut} T`   |
  | `&{,mut} [u8]`      | `&{,mut} str` |
  | `Vec<u8>`           | `String`      |
  | `Box<[u8]>`         | `String`      |
  | `*mut T`            | `Box<T>`      |
  | `*const T`          | `{Arc,Rc}<T>` |

[crate]:       https://crates.io/crates/uncon
[crate-badge]: https://img.shields.io/crates/v/uncon.svg

[derive-log]: https://github.com/nvzqz/uncon-rs/blob/master/derive/CHANGELOG.md
