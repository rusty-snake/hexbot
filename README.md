# Hexbot :construction: <br> [![top lang]][rust] ![rust 2018] ![rustc 1.31+] [![GPLv3+]][COPYING] ![version]

[top lang]: https://img.shields.io/github/languages/top/rusty-snake/hexbot.svg?logo=rust
[rust]: https://www.rust-lang.org/
[rustc 1.31+]: https://img.shields.io/badge/rustc-1.31+-blue.svg
[rust 2018]: https://img.shields.io/badge/rust--edition-2018-blue.svg
[GPLv3+]: https://img.shields.io/github/license/rusty-snake/hexbot.svg?color=darkred
[COPYING]: COPYING
[version]: https://img.shields.io/github/tag/rusty-snake/hexbot.svg?label=lastet%20release

My solution for: https://noopschallenge.com/challenges/hexbot

## Installing

### Dependencies (only for building)

cargo, rustc, pkg-config, openssl

| Distro                   | command(s)                                                                         |
| ------------------------ | ---------------------------------------------------------------------------------- |
| Arch Linux               | `sudo pacman -S rust openssl pkg-config`                                           |
| Debian 9                 | `sudo apt install pkg-config libssl-dev`<br>`curl https://sh.rustup.rs -sSf \| sh` |
| Debian 10+, Ubuntu, Mint | `sudo apt install cargo pkg-config libssl-dev`                                     |
| Fedora                   | `sudo dnf install cargo openssl-devel pkg-config`                                  |
| OpenSUSE                 | `sudo zypper install cargo openssl-devel pkg-config`                               |

### Get the source code

```bash
git clone https://github.com/rusty-snake/hexbot.git
cd hexbot
```

#### Stable code

Should not be necessary because new features are created in separate branches and master only contains minor bugfixes or new versions. If it is still required: `git chekout v0.0.3` _or whatever the lastet version is_.

### Compile & Run

```
$ cargo run --release
    Updating crates.io index
   ...
   Compiling hexbot v0.0.6 (/home/rusty-snake/hexbot)
    Finished release [optimized] target(s) in 4m 2s
     Running `target/release/hexbot`
A hexbot with five colors: [#1F13ED, #84769D, #4D052C, #8F64C5, #68C071]
The sum of all red values: 487
A hexbot with five colors and coordiantes: [#9A1184-(35|85), #C223F0-(95|79), #F7602A-(93|51), #9081C9-(48|51), #A18FAC-(12|50)]
The second color at position (95|79) has a blue component of 0.94%.
```

#### Compile only

```
$ cargo build --release
```

#### documentation

```
$ cargo doc --no-deps --open
```

## Hacking

To write your own program that uses this code to request the Hexbot API so you can focus on further processing,
copy [`src/request_api.rs`](src/request_api.rs) into `src/request_api.rs` in your project and paste the following to your files.

`Cargo.toml`:
```toml
[dependencies]
reqwest = "0.9.18"
serde = { version = "1.0", default_features = false, features = ["derive"] }
tint = "1.0.1"

[features]
# description() on Error types is deprecated, if you still need it add
# `--feature="ErrorDescription"` to the cargo command.
ErrorDescription = []
```

`src/main.rs`:
```rust
// required
pub mod request_api;

//
// Example
//

use request_api::*;

fn main {
    let hexbot = fetch(5).unwrap();
    println!("A hexbot with five colors: {}", hexbot);

    let mut red_sum = 0;
    for color in hexbot.colors() {
        red_sum += color.to_rgb255().0 as i32;
    }
    println!("The sum of all red values: {}", red_sum);

    let hexbot_with_coordinates = fetch_with_coordinates(5, 100, 100).unwrap();
    println!(
        "A hexbot with five colors and coordiantes: {}",
        hexbot_with_coordinates
    );

    let coordinate = hexbot_with_coordinates.coordinates().unwrap()[1];
    let color = hexbot_with_coordinates.colors()[1];
    println!(
        "The second color at position ({x}|{y}) has a blue component of {blue:.2}%.",
        blue = color.blue,
        x = coordinate.0,
        y = coordinate.1
    );
}
```

For the next steps, see the [documentation](#documentation).

## Changelog

```markdown
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

[0.0.6]: https://github.com/rusty-snake/hexbot/tree/v0.0.6
```

For the full Changelog see [CHANGELOG.md](CHANGELOG.md).

## License

[GPL-3.0-or-later](COPYING)

```
Copyright (C) 2019 rusty-snake <print_hello_world+License@protonmail.com>

This file is part of rusty-snake's hexbot solution

rusty-snake's hexbot solution is free software: you can redistribute
it and/or modify it under the terms of the GNU General Public License
as published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

rusty-snake's hexbot solution is distributed in the hope that it
will be useful, but WITHOUT ANY WARRANTY; without even the implied
warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
```
