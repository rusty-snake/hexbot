# [HEXBOT](https://noopschallenge.com/challenges/hexbot)

My solution for: https://noopschallenge.com/challenges/hexbot


## Installing

### Dependencies (only for building)

**cargo, rustc, pkg-config, openssl**

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

**Alternative**

```bash
wget -O hexbot.tar.gz https://github.com/rusty-snake/hexbot/archive/master.tar.gz
# or
curl -O hexbot.tar.gz https://github.com/rusty-snake/hexbot/archive/master.tar.gz
tar -xvzf hexbot.tar.gz
cd hexbot-master
```

### Compile & Run

```
$ cargo run --release
   Compiling semver-parser v0.7.0
   ...
   Compiling hexbot v0.0.1 (/home/rusty-snake/hexbot)
    Finished release [optimized] target(s) in 8m 5s
     Running `target/release/hexbot`
[src/main.rs:24] hexbot = Hexbot {
    colors: [
        Color {
            value: "#95FB4B",
        },
    ],
}
```

**Compile only**

```
$ cargo build --release
```

**debug build**

```
$ cargo build
$ cargo run
```

## Changelog

```markdown
## [0.0.1]
### Added
 - Support for requesting, parsing and printing a hexbot request without parameters.
 - All stuff around a project like README, LICENSE, .gitignore, ...

[0.0.1]: https://github.com/rusty-snake/hexbot/tree/v0.0.1
```

For the full Changelog see [CHANGELOG.md](CHANGELOG.md).

## License

[GPL-3.0-or-later](COPYING)

```
Copyright (C) 2019 rusty-snake <print_hello_world+License@protonmail.com>

This file is part of my hexbot solution

my hexbot solution is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

my hexbot solution is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
```
