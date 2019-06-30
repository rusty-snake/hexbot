/*
 * Copyright (C) 2019 rusty-snake <print_hello_world+License@protonmail.com>
 *
 * This file is part of my hexbot solution
 *
 * my hexbot solution is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * my hexbot solution is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use serde::Deserialize;
use std::fmt;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Color {
    value: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Hexbot {
    colors: [Color; 1],
}

impl Hexbot {
    #[inline]
    pub fn get_color(&self) -> &str {
        &self.colors[0].value
    }
}

impl fmt::Display for Hexbot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.colors[0].value)
    }
}

pub fn get_hexbot() -> Result<Hexbot, reqwest::Error> {
    reqwest::get("https://api.noopschallenge.com/hexbot")?.json::<Hexbot>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_color() {
        let color = Color {
            value: "#FFFFFF".to_string(),
        };
        let hexbot = Hexbot { colors: [color; 1] };

        assert_eq!(hexbot.get_color(), "#FFFFFF");
    }
}
