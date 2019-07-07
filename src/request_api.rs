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

//! Request the [Hexbot-API] and process the response.
//!
//! # Examples
//!
//! ```
//! let hexbot: Hexbot = fetch(3).unwrap();
//! println!("{}", hexbot); // [#RRGGBB, #RRGGBB, #RRGGBB]
//! let colors: Vec<&tint::Color> = hexbot.colors();
//! dbg!(&colors);           // &vec![&Color, &Color, &Color]
//! let color: &tint::Color = colors.first().unwrap();
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
use std::error;
use std::fmt;
use tint::Color;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
struct Dot {
    #[serde(rename = "value", deserialize_with = "deserialize_color")]
    color: Color,
}

/// Deserialized response
#[derive(Debug, Deserialize, PartialEq)]
pub struct Hexbot {
    colors: Vec<Dot>,
}

impl Hexbot {
    /// Returns a vector containing a reference to all colors.
    pub fn colors(&self) -> Vec<&Color> {
        self.colors.iter().map(|dot| &dot.color).collect()
    }
}

impl fmt::Display for Hexbot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let len = self.colors.len() - 1;
        for (counter, Dot { color }) in self.colors.iter().enumerate() {
            if counter == len {
                write!(f, "{}]", color.to_hex().to_uppercase())?;
            } else {
                write!(f, "{}, ", color.to_hex().to_uppercase())?;
            }
        }
        Ok(())
    }
}

fn deserialize_color<'d, D: Deserializer<'d>>(deser: D) -> Result<Color, D::Error> {
    Ok(Color::from_hex(&String::deserialize(deser)?))
}

/// Fetch the given `count` of colors from the Hexbot API.
///
/// `count` must be between 1 and 1000.
pub fn fetch(count: i32) -> Result<Hexbot, Error> {
    if 0 < count && count <= 1000 {
        Ok(reqwest::get(&format!(
            "https://api.noopschallenge.com/hexbot?count={}",
            count
        ))?
        .json()?)
    } else {
        Err(Error::CountOutOfRange)
    }
}

// Error

/// Request-API Error
#[derive(Debug)]
pub enum Error {
    /// Contains a reqwest error
    Reqwest(reqwest::Error),
    /// Occurs when the count is higher then 1000 or lower then 1.
    CountOutOfRange,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Reqwest(ref err) => err.fmt(f),
            Error::CountOutOfRange => write!(f, "count must be between 1 and 1000"),
        }
    }
}

impl error::Error for Error {
    // Usage of description is soft-deprecated; use Display
    #[cfg(ErrorDescription)]
    fn description(&self) -> &str {
        match *self {
            Error::Reqwest(ref err) => err.description(),
            Error::CountOutOfRange => "count must be between 1 and 1000",
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Reqwest(ref err) => err.source(),
            Error::CountOutOfRange => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colors() {
        let hexbot = Hexbot {
            colors: vec![
                Dot {
                    color: Color::new(1.0, 1.0, 1.0, 1.0),
                },
                Dot {
                    color: Color::new(0.0, 0.0, 0.0, 1.0),
                },
            ],
        };

        assert_eq!(
            hexbot.colors(),
            vec![
                &Color::new(1.0, 1.0, 1.0, 1.0),
                &Color::new(0.0, 0.0, 0.0, 1.0),
            ]
        );
    }
}
