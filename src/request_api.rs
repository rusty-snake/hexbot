/*
 * Copyright (C) 2019 rusty-snake <print_hello_world+License@protonmail.com>
 *
 * This file is part of rusty-snake's hexbot solution
 *
 * rusty-snake's hexbot solution is free software: you can redistribute
 * it and/or modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * rusty-snake's hexbot solution is distributed in the hope that it
 * will be useful, but WITHOUT ANY WARRANTY; without even the implied
 * warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! All code to request the [Hexbot-API] and process the response.
//!
//! # Examples
//!
//! ```
//! let hexbot: Hexbot = fetch().unwrap();
//! println!("{}", hexbot); // #RRGGBB
//! let color: &tint::Color = hexbot.color();
//! dbg!(color.red);         // f64: red value
//! dbg!(color.green);       // f64: green value
//! dbg!(color.blue);        // f64: red value
//! dbg!(color.to_rgb1());   // (f64, f64, f64): (R, G, B)
//! dbg!(color.to_rgb255()); // (u8, u8, u8): (R, G, B)
//! dbg!(color.to_hex());    // String: "#rrggbb"
//! dbg!(color.to_hsv());    // (f64, f64, f64): (H, S, V)
//! dbg!(color.to_hsl());    // (f64, f64, f64): (H, S, L)
//! dbg!(color.to_yiq());    // (f64, f64, f64): (Y, I, Q)
//! ```
//!
//! [Hexbot-API]: https://noopschallenge.com/challenges/hexbot

use serde::{Deserialize, Deserializer};
use std::fmt;
use tint::Color;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
struct Dot {
    #[serde(deserialize_with = "deserialize_color", rename = "value")]
    color: Color,
}

/// Deserialized response.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct Hexbot {
    colors: [Dot; 1],
}

impl Hexbot {
    #[inline]
    /// Get the color from the response.
    ///
    /// The return type is `&tint::Color`, see its [documentation]
    /// for further processing.
    ///
    /// [documentation]: https://docs.rs/tint/1.0.1/tint/struct.Color.html
    pub fn color(&self) -> &Color {
        &self.colors[0].color
    }
}

impl fmt::Display for Hexbot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.colors[0].color.to_hex().to_uppercase())
    }
}

fn deserialize_color<'d, D: Deserializer<'d>>(deser: D) -> Result<Color, D::Error> {
    Ok(Color::from_hex(&String::deserialize(deser)?))
}

/// Send a request and process the response.
pub fn fetch() -> Result<Hexbot, reqwest::Error> {
    reqwest::get("https://api.noopschallenge.com/hexbot")?.json::<Hexbot>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_color() {
        let color = Dot {
            color: Color::new(1.0, 1.0, 1.0, 1.0),
        };
        let hexbot = Hexbot { colors: [color; 1] };

        assert_eq!(hexbot.color(), &Color::new(1.0, 1.0, 1.0, 1.0));
    }
}
