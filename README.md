# Hexbot :construction: <br> [![top language: rust]][rust-lang] ![rust 2018] ![rustc 1.34+] [![GPLv3+]][COPYING] ![tag]

[top language: rust]: https://img.shields.io/github/languages/top/rusty-snake/hexbot.svg?logo=rust
[rust-lang]: https://www.rust-lang.org/
[rustc 1.34+]: https://img.shields.io/badge/rustc-1.34+-blue.svg?logo=rust
[rust 2018]: https://img.shields.io/badge/rust--edition-2018-blue.svg?logo=rust
[GPLv3+]: https://img.shields.io/github/license/rusty-snake/hexbot.svg?color=darkred
[COPYING]: COPYING
[tag]: https://img.shields.io/github/tag/rusty-snake/hexbot.svg

[![Travis CI Status](https://badgen.net/travis/rusty-snake/hexbot/master?icon=travis&label=build)](https://travis-ci.com/rusty-snake/hexbot)
[![Dependabot Status](https://api.dependabot.com/badges/status?host=github&repo=rusty-snake/hexbot)](https://dependabot.com)
![actively developed](https://badgen.net/badge/maintenance/actively-developed/green)

My solution for: https://noopschallenge.com/challenges/hexbot

## Installing

### Install [rust](https://www.rust-lang.org/)

| Distro               | command(s)                  |
| -------------------- | --------------------------- |
| Arch Linux           | `sudo pacman -S rust`       |
| Debian, Ubuntu, Mint | `sudo apt install cargo`    |
| Fedora               | `sudo dnf install cargo`    |
| OpenSUSE             | `sudo zypper install cargo` |

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
   Compiling hexbot v0.0.9 (/home/rusty-snake/hexbot)
    Finished release [optimized] target(s) in 4m 2s
     Running `target/release/hexbot`
A hexbot with twenty colors: [#2A4F9D, #CFE0FC, #6801DD, #A70B33, #F4D8CD, #F41829, #EA0037, #38F496, #C7E8A9, #3A1CC5, #869FB2, #CC3A91, #FB1FFE, #FEAB77, #5168C1, #EBDF47, #D7608B, #AFBA33, #AD0A50, #A2F169]
The sum of all red values: 3439
A hexbot with five colors and coordiantes: [#AC83C4-(5|13), #EFFC3B-(5|18), #48AF07-(6|99), #B1B2B8-(48|80), #726B34-(19|66)]
The second color at position (5|18) has a blue component of 0.23%.
A hexbot with three hues of blue: [#000061, #0000A6, #000095]
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

To write your own program that uses this code to request the Hexbot API so you can focus on further processing,
copy [`src/request_api.rs`](src/request_api.rs) into `src/request_api.rs` in your project and paste the following to your files.

`Cargo.toml`:
```toml
[dependencies]
tint = "1.0.1"

[dependencies.reqwest]
version = "0.9"
default_features = false
features = ["rustls-tls"]

[dependencies.serde]
version = "1.0"
default_features = false
features = ["derive"]

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

use request_api::{fetch, fetch_with_coordinates, Error};

fn main() -> Result<(), Error> {
    //
    // New hexbot with the parameter `count=20`.
    //
    let hexbot = fetch(20, None)?;
    println!("A hexbot with twenty colors: {}", hexbot);

    // sum all red values

    let mut red_sum = 0;
    for color in hexbot.colors() {
        red_sum += i32::from(color.to_rgb255().0);
    }
    println!("The sum of all red values: {}", red_sum);

    //
    // New hexbot with the parameters `count=5&width=100&heigth=100`.
    //
    let hexbot_with_coordinates = fetch_with_coordinates(5, 100, 100, None)?;
    println!(
        "A hexbot with five colors and coordiantes: {}",
        hexbot_with_coordinates
    );

    // Show the blue component of the second color and print its position.

    let coordinate = hexbot_with_coordinates.coordinates().unwrap()[1];
    let color = hexbot_with_coordinates.colors()[1];
    println!(
        "The second color at position ({x}|{y}) has a blue component of {blue:.2}%.",
        blue = color.blue,
        x = coordinate.0,
        y = coordinate.1
    );

    //
    // New hexbot with the parameters `count=3&seed=000000,0000FF`.
    //
    let hexbot_blue_only = fetch(3, Some(&[0x_00_00_00, 0x_00_00_FF]))?;
    println!("A hexbot with three hues of blue: {}", hexbot_blue_only);

    Ok(())
}
```

For the next steps, see the [documentation](#documentation).

## Changelog

```markdown
## [0.0.9] - 2019-09-04
maintenance release
 * fixing various non-code things
 * updating dependencies

[0.0.9]: https://github.com/rusty-snake/hexbot/tree/v0.0.9
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
