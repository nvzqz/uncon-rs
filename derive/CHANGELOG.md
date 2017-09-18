# Changelog [![Crates.io][crate-badge]][crate]
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 1.1.0 - 2017-09-18
### Added
- Derive `From<T>` via `FromUnchecked<T>` for C-like enums
  - Done via `#[uncon(impl_from)]`

## 1.0.4 - 2017-09-18
### Fixed
- Allow multiple `#[uncon]` attributes.

## 1.0.3 - 2017-09-17
### Added
- Derive `FromUnchecked` for other integer types if the derived type is also an
  integer.
  - Done via `#[uncon(other(u8, i8, ...))]`
  - This is implemented via a simple `as` cast. In some cases, this may not make
    sense, so it is recommended to manually implement the conversions for such
    types.

## 1.0.2 - 2017-09-12
### Fixed
- Fixed `uncon_derive` dependency on `uncon` for dev builds:

  ```
  error: Package `uncon_derive v1.0.1` does not have these features: `uncon`
  ```

## 1.0.1 - 2017-09-12
### Fixed
- Fixed derive for `#[repr({u,i}size)]`

## 1.0.0 - 2017-09-11
### Added
- `FromUnchecked` derive for:
    - Structs with a single field
    - C-like enums with `#[repr]` attribute

[crate]:       https://crates.io/crates/uncon_derive
[crate-badge]: https://img.shields.io/crates/v/uncon_derive.svg
