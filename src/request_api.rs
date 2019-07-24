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
//! ## 1
//! ```
//! let hexbot: Hexbot = fetch(3, None).unwrap();
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
//! ## 2
//! ```
//! let hexbot = fetch(5, None).unwrap();
//! println!("A hexbot with five colors: {}", hexbot);
//!
//! let mut red_sum = 0;
//! for color in hexbot.colors() {
//!     red_sum += color.to_rgb255().0 as i32;
//! }
//! println!("The sum of all red values: {}", red_sum);
//!
//! let hexbot_with_coordinates = fetch_with_coordinates(5, 100, 100, None).unwrap();
//! println!(
//!     "A hexbot with five colors and coordiantes: {}",
//!     hexbot_with_coordinates
//! );
//!
//! let coordinate = hexbot_with_coordinates.coordinates().unwrap()[1];
//! let color = hexbot_with_coordinates.colors()[1];
//! println!(
//!     "The second color at position ({x}|{y}) has a blue component of {blue:.2}%.",
//!     blue = color.blue,
//!     x = coordinate.0,
//!     y = coordinate.1
//! );
//! ```
//!
//! [Hexbot-API]: https://noopschallenge.com/challenges/hexbot

use serde::{Deserialize, Deserializer};
use std::error;
use std::fmt;
use std::fmt::Write;
use tint::Color;

static API_ENDPOINT: &str = "https://api.noopschallenge.com/hexbot";

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
struct Dot {
    #[serde(rename = "value", deserialize_with = "deserialize_color")]
    color: Color,
    coordinates: Option<Coordinate>,
}

/// Deserialized response
#[derive(Clone, Debug, PartialEq)]
pub struct Hexbot {
    colors: Vec<Dot>,
}

impl Hexbot {
    /// Returns a vector containing a reference to all colors.
    pub fn colors(&self) -> Vec<&Color> {
        self.colors.iter().map(|dot| &dot.color).collect()
    }

    /// Get the coordiantes of all colors.
    ///
    /// Returns `None` if the hexbot doesn't have coordinates,
    /// otherwise it returns the coordinates in the form `(x, y)`
    /// in an vector.
    ///
    /// # Example
    ///
    /// ```
    /// let (x, y) = hexbot.coordinates().unwrap().get(1).unwrap();
    /// println!("x: {}, y: {}", x, y);
    /// ```
    pub fn coordinates(&self) -> Option<Vec<(i32, i32)>> {
        let mut vec = Vec::new();
        for dot in &self.colors {
            let Coordinate { x, y } = dot.coordinates?;
            vec.push((x, y));
        }
        Some(vec)
    }

    /// Returns `true` if the hexbot has coordinates, otherwise `false`.
    pub fn has_coordinates(&self) -> bool {
        !(self.colors[0].coordinates == None)
    }
}

impl fmt::Display for Hexbot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        // NOTE: When editing this line, keep in mind that it protect
        // the `unwrap`s below.
        if self.has_coordinates() {
            for Dot { color, coordinates } in &self.colors {
                write!(
                    s,
                    "{}-({}|{}), ",
                    color.to_hex().to_uppercase(),
                    // NOTE: These `unwrap`s are safe because of the
                    // `if self.has_coordinates()` above.
                    coordinates.unwrap().x,
                    coordinates.unwrap().y
                )?;
            }
        } else {
            for Dot { color, .. } in &self.colors {
                write!(s, "{}, ", color.to_hex().to_uppercase())?;
            }
        }
        // HACK:FIXME: Use a better solution to remove the last two chars.
        s.pop();
        s.pop();
        write!(f, "[{}]", s)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum ResponseKind {
    Colors(Vec<Dot>),
    Message(String),
    Error(String),
}

fn deserialize_color<'de, D>(deser: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Color::from_hex(&String::deserialize(deser)?))
}

fn parse_seed(input_seed: &[i32]) -> Result<String, Error> {
    if input_seed.is_empty() {
        return Err(Error::EmptySeed);
    }
    if input_seed.len() > 10 {
        return Err(Error::SeedToLong);
    }
    let mut processed_seed = String::new();
    for &color in input_seed {
        if !(0x_00_00_00 <= color && color <= 0x_FF_FF_FF) {
            return Err(Error::InvalidSeedColor);
        }
        write!(&mut processed_seed, "{:06X},", color)?;
    }
    processed_seed.pop();
    Ok(processed_seed)
}

/// Fetch the given `count` of colors from the Hexbot API.
///
/// `count` must be between 1 and 1000.
pub fn fetch(count: i32, seed: Option<&[i32]>) -> Result<Hexbot, Error> {
    if !(1 <= count && count <= 1000) {
        return Err(Error::CountOutOfRange);
    }
    let response: ResponseKind = match seed {
        None => reqwest::get(&format!("{}?count={}", API_ENDPOINT, count))?.json()?,
        Some(seed) => reqwest::get(&format!(
            "{}?count={}&seed={}",
            API_ENDPOINT,
            count,
            parse_seed(seed)?
        ))?
        .json()?,
    };
    match response {
        ResponseKind::Colors(colors) => Ok(Hexbot { colors }),
        ResponseKind::Message(message) => Err(Error::HexbotMessage(message)),
        ResponseKind::Error(error) => Err(Error::HexbotError(error)),
    }
}

/// Fetch the given count of colors in the given range of coordinates from the Hexbot API.
///
/// ## Thresholds:
///  * `1 <= count <= 1,000`
///  * `10 <= width <= 100,000`
///  * `10 <= height <= 100,000`
pub fn fetch_with_coordinates(
    count: i32,
    width: i32,
    height: i32,
    seed: Option<&[i32]>,
) -> Result<Hexbot, Error> {
    if !(1 <= count && count <= 1000) {
        return Err(Error::CountOutOfRange);
    }
    if !(10 <= width && width <= 100_000) || !(10 <= height && height <= 100_000) {
        return Err(Error::WidthHeightOutOfRange);
    }
    let response: ResponseKind = match seed {
        None => reqwest::get(&format!(
            "{}?count={}&width={}&height={}",
            API_ENDPOINT, count, width, height
        ))?
        .json()?,
        Some(seed) => reqwest::get(&format!(
            "{}?count={}&width={}&height={},seed={}",
            API_ENDPOINT,
            count,
            width,
            height,
            parse_seed(seed)?
        ))?
        .json()?,
    };
    match response {
        ResponseKind::Colors(colors) => Ok(Hexbot { colors }),
        ResponseKind::Message(message) => Err(Error::HexbotMessage(message)),
        ResponseKind::Error(error) => Err(Error::HexbotError(error)),
    }
}

// Error

/// Request-API Error
#[derive(Debug)]
pub enum Error {
    /// Contains a reqwest error
    Reqwest(reqwest::Error),
    /// Contains `std::fmt::Error`.
    Fmt(fmt::Error),
    /// Occurs wheb the Hexbot-API responded with a message.
    HexbotMessage(String),
    /// Occurs wheb the Hexbot-API responded with a error message.
    HexbotError(String),
    /// Occurs when the count is higher then 1000 or lower then 1.
    CountOutOfRange,
    /// Occurs when width and/or height is higher then 100,000 or lower then 10.
    WidthHeightOutOfRange,
    /// Occurs when the given seed is empty.
    EmptySeed,
    /// Occurs when the given seed has more than 10 colors.
    SeedToLong,
    /// Occurs when a color in seed is lower than 0 or higher then 0xFFFFFF.
    InvalidSeedColor,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Reqwest(err) => err.fmt(f),
            Error::Fmt(err) => err.fmt(f),
            Error::HexbotMessage(message) => write!(f, "responded with: {}", message),
            Error::HexbotError(error) => write!(f, "responded with: {}", error),
            Error::CountOutOfRange => write!(f, "count must be between 1 and 1000"),
            Error::WidthHeightOutOfRange => {
                write!(f, "width and height must be between 10 and 100,000")
            }
            Error::EmptySeed => write!(f, "seed must contain at least one color"),
            Error::SeedToLong => write!(f, "seed must not contain more than 10 colors"),
            Error::InvalidSeedColor => write!(
                f,
                "the i32 in seed must be between 0 and 16777215 (hex: FFFFFF)"
            ),
        }
    }
}

impl error::Error for Error {
    // Usage of description is soft-deprecated; use Display
    #[cfg(ErrorDescription)]
    fn description(&self) -> &str {
        match self {
            Error::Reqwest(err) => err.description(),
            Error::Fmt(err) => err.description(),
            Error::HexbotMessage(message) => "responded with: " + message,
            Error::HexbotError(error) => "responded with: " + error,
            Error::CountOutOfRange => "count must be between 1 and 1000",
            Error::WidthHeightOutOfRange => "width and height must be between 10 and 100,000",
            Error::EmptySeed => "seed must contain at least one color",
            Error::SeedToLong => "seed must not contain more than 10 colors",
            Error::InvalidSeedColor => {
                "the i32 in seed must be between 0 and 16777215 (hex: FFFFFF)"
            }
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Reqwest(err) => err.source(),
            Error::Fmt(err) => err.source(),
            Error::CountOutOfRange
            | Error::WidthHeightOutOfRange
            | Error::EmptySeed
            | Error::SeedToLong
            | Error::InvalidSeedColor
            | Error::HexbotMessage(_)
            | Error::HexbotError(_) => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error::Fmt(err)
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
                    coordinates: None,
                },
                Dot {
                    color: Color::new(0.0, 0.0, 0.0, 1.0),
                    coordinates: None,
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

    #[test]
    fn test_coordinates() {
        let hexbot = Hexbot {
            colors: vec![
                Dot {
                    color: Color::new(1.0, 1.0, 1.0, 1.0),
                    coordinates: Some(Coordinate { x: 100, y: 100 }),
                },
                Dot {
                    color: Color::new(1.0, 1.0, 1.0, 1.0),
                    coordinates: Some(Coordinate { x: 500, y: 30 }),
                },
            ],
        };
        assert_eq!(hexbot.coordinates(), Some(vec![(100, 100), (500, 30)]));

        let hexbot = Hexbot {
            colors: vec![
                Dot {
                    color: Color::new(1.0, 1.0, 1.0, 1.0),
                    coordinates: None,
                },
                Dot {
                    color: Color::new(0.0, 0.0, 0.0, 1.0),
                    coordinates: None,
                },
            ],
        };
        assert_eq!(hexbot.coordinates(), None);
    }

    #[test]
    fn test_has_coordinates() {
        let hexbot = Hexbot {
            colors: vec![Dot {
                color: Color::new(1.0, 1.0, 1.0, 1.0),
                coordinates: None,
            }],
        };
        assert_eq!(hexbot.has_coordinates(), false);

        let hexbot = Hexbot {
            colors: vec![Dot {
                color: Color::new(1.0, 1.0, 1.0, 1.0),
                coordinates: Some(Coordinate { x: 500, y: 30 }),
            }],
        };
        assert_eq!(hexbot.has_coordinates(), true);
    }

    #[test]
    fn test_fetch() {
        // check that 0 results in Error::CountOutOfRange
        assert!(match fetch(0, None) {
            Err(Error::CountOutOfRange) => true,
            _ => false,
        });
        // check that 1001 results in Error::CountOutOfRange
        assert!(match fetch(1001, None) {
            Err(Error::CountOutOfRange) => true,
            _ => false,
        });
        // check that 1 does not result in Error::CountOutOfRange
        assert!(match fetch(1, None) {
            Err(Error::CountOutOfRange) => false,
            _ => true,
        });
        // check that 1000 does not result in Error::CountOutOfRange
        assert!(match fetch(1000, None) {
            Err(Error::CountOutOfRange) => false,
            _ => true,
        });
        // check for an empty seed
        assert!(if let Err(Error::EmptySeed) = fetch(1, Some(&[])) {
            true
        } else {
            false
        });
        // check that a seed with 10 colors is valid
        assert!(
            if let Err(Error::SeedToLong) = fetch(1, Some(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0])) {
                false
            } else {
                true
            }
        );
        // check that a seed with 11 colors is invalid
        assert!(
            if let Err(Error::SeedToLong) = fetch(1, Some(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])) {
                true
            } else {
                false
            }
        );
        // check that a negative color is invalid
        assert!(
            if let Err(Error::InvalidSeedColor) = fetch(1, Some(&[-1])) {
                true
            } else {
                false
            }
        );
        // check that 0 is valid
        assert!(if let Err(Error::InvalidSeedColor) = fetch(1, Some(&[0])) {
            false
        } else {
            true
        });
        // check that 0x_FF_FF_FF is valid
        assert!(
            if let Err(Error::InvalidSeedColor) = fetch(1, Some(&[16777215])) {
                false
            } else {
                true
            }
        );
        // check that 0x_FF_FF_FF + 1 is invalid
        assert!(
            if let Err(Error::InvalidSeedColor) = fetch(1, Some(&[16777216])) {
                true
            } else {
                false
            }
        );
        // Check that the Ok() data type of fetch is hexbot.
        // NOTE: That this is done during compilation, not during execution.
        #[allow(unused_variables)]
        match fetch(3, None) {
            Ok(hb) => {
                let hexbot: Hexbot = hb;
            }
            Err(err) => (),
        }
    }

    #[test]
    fn test_fetch_with_coordinates() {
        // check that 0 results in Error::CountOutOfRange
        assert!(match fetch_with_coordinates(0, 100, 100, None) {
            Err(Error::CountOutOfRange) => true,
            _ => false,
        });
        // check that 1001 results in Error::CountOutOfRange
        assert!(match fetch_with_coordinates(1001, 100, 100, None) {
            Err(Error::CountOutOfRange) => true,
            _ => false,
        });
        // check that 1 does not result in Error::CountOutOfRange
        assert!(match fetch_with_coordinates(1, 100, 100, None) {
            Err(Error::CountOutOfRange) => false,
            _ => true,
        });
        // check that 1000 does not result in Error::CountOutOfRange
        assert!(match fetch_with_coordinates(1000, 100, 100, None) {
            Err(Error::CountOutOfRange) => false,
            _ => true,
        });
        // check that width=9 ends in Error::WidthHeightOutOfRange
        assert!(
            if let Err(Error::WidthHeightOutOfRange) = fetch_with_coordinates(1, 9, 100, None) {
                true
            } else {
                false
            }
        );
        // check that width=10 not ends in Error::WidthHeightOutOfRange
        assert!(
            if let Err(Error::WidthHeightOutOfRange) = fetch_with_coordinates(1, 10, 100, None) {
                false
            } else {
                true
            }
        );
        // check that width=100,000 not ends in Error::WidthHeightOutOfRange
        assert!(if let Err(Error::WidthHeightOutOfRange) =
            fetch_with_coordinates(1, 100_000, 100, None)
        {
            false
        } else {
            true
        });
        // check that width=100,001 ends in Error::WidthHeightOutOfRange
        assert!(if let Err(Error::WidthHeightOutOfRange) =
            fetch_with_coordinates(1, 100_001, 100, None)
        {
            true
        } else {
            false
        });
        // check that height=9 ends in Error::WidthHeightOutOfRange
        assert!(
            if let Err(Error::WidthHeightOutOfRange) = fetch_with_coordinates(1, 100, 9, None) {
                true
            } else {
                false
            }
        );
        // check that height=10 not ends in Error::WidthHeightOutOfRange
        assert!(
            if let Err(Error::WidthHeightOutOfRange) = fetch_with_coordinates(1, 100, 10, None) {
                false
            } else {
                true
            }
        );
        // check that height=100,000 not ends in Error::WidthHeightOutOfRange
        assert!(if let Err(Error::WidthHeightOutOfRange) =
            fetch_with_coordinates(1, 100, 100_000, None)
        {
            false
        } else {
            true
        });
        // check that height=100,001 ends in Error::WidthHeightOutOfRange
        assert!(if let Err(Error::WidthHeightOutOfRange) =
            fetch_with_coordinates(1, 100, 100_001, None)
        {
            true
        } else {
            false
        });
        // check for an empty seed
        assert!(
            if let Err(Error::EmptySeed) = fetch_with_coordinates(1, 100, 100, Some(&[])) {
                true
            } else {
                false
            }
        );
        // check that a seed with 10 colors is valid
        assert!(if let Err(Error::SeedToLong) =
            fetch_with_coordinates(1, 100, 100, Some(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
        {
            false
        } else {
            true
        });
        // check that a seed with 11 colors is invalid
        assert!(if let Err(Error::SeedToLong) =
            fetch_with_coordinates(1, 100, 100, Some(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
        {
            true
        } else {
            false
        });
        // check that a negative color is invalid
        assert!(if let Err(Error::InvalidSeedColor) =
            fetch_with_coordinates(1, 100, 100, Some(&[-1]))
        {
            true
        } else {
            false
        });
        // check that 0 is valid
        assert!(if let Err(Error::InvalidSeedColor) =
            fetch_with_coordinates(1, 100, 100, Some(&[0]))
        {
            false
        } else {
            true
        });
        // check that 0x_FF_FF_FF is valid
        assert!(if let Err(Error::InvalidSeedColor) =
            fetch_with_coordinates(1, 100, 100, Some(&[16777215]))
        {
            false
        } else {
            true
        });
        // check that 0x_FF_FF_FF + 1 is invalid
        assert!(if let Err(Error::InvalidSeedColor) =
            fetch_with_coordinates(1, 100, 100, Some(&[16777216]))
        {
            true
        } else {
            false
        });
        // Check that the Ok() data type of fetch_with_coordinates is hexbot.
        // NOTE: That this is done during compilation, not during execution.
        #[allow(unused_variables)]
        match fetch_with_coordinates(3, 100, 100, None) {
            Ok(hb) => {
                let hexbot: Hexbot = hb;
            }
            Err(err) => (),
        }
    }
}
