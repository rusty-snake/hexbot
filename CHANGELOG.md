# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased
### Added
 * `Seed::add`
 * impl ops::{Add,Sub} for Coordinates

## [0.0.10] - 2019-09-28
### Changed
 * min rustc: 1.37.0

### Added
  * library to use the hexbot API

### Removed
  * feature: `ErrorDescription`
  * `src/request_api.rs`

## [0.0.9] - 2019-09-04
maintenance release
 * fixing various non-code things
 * updating dependencies

## [0.0.8]
### Added
 * Support for error and message response
 * `.rustfmt.toml`

### Changed
 * Switch from openssl to rustls

## [0.0.7]
### Added
 * more tests.
 * support for hexbots `seed` parameter
   * add `seed: Option<&[i32]>` to `fetch()` and `fetch_with_coordinates()`
 * more Errors
   * `Fmt(fmt::Error)`
   * `EmptySeed`
   * `SeedToLong`
   * `InvalidSeedColor`

## [0.0.6]
### Added
 * `Hexbot.clone()` (derive trait).
 * support for the width and height parameters of hexbot.
   * `Hexbot.coordinates()`
   * `Hexbot.has_coordinates()`
   * `request_api::fetch_with_coordinates()`
   * `Error::WidthHeightOutOfRange`

### Changed
 * `fmt::Display` for `Hexbot`.
 * Improved docs & Hacking.

## [0.0.5]
### Added
 * custom error type.
 * more tests.
 * `request_api::Hexbot.colors()`.
 * support for the count parameter of hexbot.

### Changed
 * output of `fmt::Display` for Hexbot.
 * `request_api::fetch()`
   * old: `request_api::fetch() -> Result<Hexbot, reqwest::Error>`
   * new: `request_api::fetch(count: i32) -> Result<Hexbot, Error>`

### Removed
 * `request_api::Hexbot.color()`.

## [0.0.4]
### Added
 * documentation & hacking.

### Changed
 * Function names:
   * `get_color()` -> `color()`
   * `get_hexbot()` -> `fetch()`
 * Use `tint::Color` instead of `String` for colors.

## [0.0.3]
### Added
 * tests (see c461bb9).

## [0.0.2]
### Added
 - `get_color()` methode to `request_api::Hexbot`.
 - `impl`lementation for `fmt::Display` to `request_api::Hexbot`.

### Removed
 - `pub`lic fields from `request_api::Hexbot`.

## [0.0.1]
### Added
 - Support for requesting, parsing and printing a hexbot request without parameters.
 - All stuff around a project like README, LICENSE, .gitignore, ...

[0.0.10]: https://github.com/rusty-snake/hexbot/tree/v0.0.10
[0.0.9]: https://github.com/rusty-snake/hexbot/tree/v0.0.9
[0.0.8]: https://github.com/rusty-snake/hexbot/tree/v0.0.8
[0.0.7]: https://github.com/rusty-snake/hexbot/tree/v0.0.7
[0.0.6]: https://github.com/rusty-snake/hexbot/tree/v0.0.6
[0.0.5]: https://github.com/rusty-snake/hexbot/tree/v0.0.5
[0.0.4]: https://github.com/rusty-snake/hexbot/tree/v0.0.4
[0.0.3]: https://github.com/rusty-snake/hexbot/tree/v0.0.3
[0.0.2]: https://github.com/rusty-snake/hexbot/tree/v0.0.2
[0.0.1]: https://github.com/rusty-snake/hexbot/tree/v0.0.1
