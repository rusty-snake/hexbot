# :construction: :crab: Hexbot :crab: :construction: <br> [![top language: rust]][rust-lang] ![rust 2018] ![rustc 1.37+] [![GPLv3+]][COPYING] ![tag]

[top language: rust]: https://img.shields.io/github/languages/top/rusty-snake/hexbot.svg?logo=rust
[rust-lang]: https://www.rust-lang.org/
[rustc 1.37+]: https://img.shields.io/badge/rustc-1.37+-blue.svg?logo=rust
[rust 2018]: https://img.shields.io/badge/rust--edition-2018-blue.svg?logo=rust
[GPLv3+]: https://img.shields.io/github/license/rusty-snake/hexbot.svg?color=darkred
[COPYING]: COPYING
[tag]: https://img.shields.io/github/tag/rusty-snake/hexbot.svg

[![Travis CI Status](https://badgen.net/travis/rusty-snake/hexbot/master?icon=travis&label=build)](https://travis-ci.com/rusty-snake/hexbot)
[![Dependabot Status](https://api.dependabot.com/badges/status?host=github&repo=rusty-snake/hexbot)](https://dependabot.com)
![actively developed](https://badgen.net/badge/maintenance/actively-developed/green)

My solution for: https://noopschallenge.com/challenges/hexbot

## Getting Started

### Install [rust](https://www.rust-lang.org/)

The canonical way to [install rust] is to use rustup:

```bash
curl https://sh.rustup.rs -sSf | sh
```

You need this on distros who do not have a supported rustc version in their packet sources.
e.g. Debian Stable, Mint, openSUSE Leap, Ubuntu, ...

On distros which have the current rustc version in their packet sources, it can be used.

| Distro                  | command                     |
| ----------------------- | --------------------------- |
| Arch Linux              | `sudo pacman -S rust`       |
| Debian Testing/Unstable | `sudo apt install cargo`    |
| Fedora                  | `sudo dnf install cargo`    |
| openSUSE Tumbleweed     | `sudo zypper install cargo` |

[install rust]: https://www.rust-lang.org/tools/install

### Get the source code

```bash
git clone https://github.com/rusty-snake/hexbot.git
cd hexbot
```

### Compile & Run

```
$ cargo run --release
    Updating crates.io index
   ...
   Compiling hexbot v0.0.11 (/home/rusty-snake/hexbot)
    Finished release [optimized] target(s) in 4m 2s
     Running `target/release/hexbot`
===== Hexbot =====
Should the count parameter be added? [yes|no] yes
What value should count have? [1-1000] 5
Should the width and height parameters be added? [yes|no] yes
What value should width have? [10-100,000] 500
What value should height have? [10-100,000] 500
[#E46AF7-(370|226), #FAFD72-(334|70), #6C1882-(440|490), #ECC44A-(451|181), #F528DF-(151|72)]
```

#### compile only

```
$ cargo build --release
```

#### documentation

```
$ cargo doc --no-deps --open
```

## Hacking

Using the hexbot library in your own project.

`Cargo.toml`:
```toml
[dependencies]
hexbot = { git = "https://github.com/rusty-snake/hexbot", tag = "v0.0.11" }
```

`src/main.rs`:
```rust
use hexbot::{Count, Hexbot, Seed, WidthHeight};

fn main() {
    let hb = Hexbot::fetch(
        Count::no(),
        WidthHeight::no(),
        &Seed::no()
    );
    println!("Hello from Hexbot: {}", hb);
}
```

For the next steps, see the [documentation](#documentation).

## Changelog

```markdown
## [0.0.11] - 2019-10-13
### Added
 * `Seed::add`
 * impl ops::{Add,Sub} for Coordinates
 * `hexbot::WidthHeight` improved replacement for `WithCoordinates` and `CoordinatesLimit`

### Changed
 * `Hexbot::fetch` accept both `WithCoordinates` and `WidthHeight`
 * old:`SeedError::NoColor` new:`SeedError::NoColor(i32)`

### Deprecated
 * `WithCoordinates` use `WidthHeight`
 * `CoordinatesLimit` use `WidthHeight`

[0.0.11]: https://github.com/rusty-snake/hexbot/tree/v0.0.11
```

For the full Changelog see [CHANGELOG.md](CHANGELOG.md).

## License

![](https://www.gnu.org/graphics/gplv3-127x51.png)

[GPL-3.0-or-later](COPYING)
