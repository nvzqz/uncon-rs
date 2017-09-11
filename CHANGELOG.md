# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

**Note:** `uncon_derive` has [its own changelog][derive-log] separate from this one.

## [Unreleased]

### Added
- `FromUnchecked` and `IntoUnchecked` traits.
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

[derive-log]: https://github.com/nvzqz/uncon-rs/blob/master/derive/CHANGELOG.md
