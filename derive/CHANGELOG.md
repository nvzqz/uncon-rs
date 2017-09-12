# Changelog [![Crates.io][crate-badge]][crate]
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
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
