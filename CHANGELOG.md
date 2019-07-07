# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.0.5]: https://github.com/rusty-snake/hexbot/tree/v0.0.5
[0.0.4]: https://github.com/rusty-snake/hexbot/tree/v0.0.4
[0.0.3]: https://github.com/rusty-snake/hexbot/tree/v0.0.3
[0.0.2]: https://github.com/rusty-snake/hexbot/tree/v0.0.2
[0.0.1]: https://github.com/rusty-snake/hexbot/tree/v0.0.1
