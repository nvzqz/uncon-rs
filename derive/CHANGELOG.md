# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Fixed
- Fixed derive for `#[repr({u,i}size)]`

## 1.0.0 - 2017-09-11
### Added
- `FromUnchecked` derive for:
    - Structs with a single field
    - C-like enums with `#[repr]` attribute
