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
//! ## 2
//! ```
//! let hexbot = fetch(5).unwrap();
//! println!("A hexbot with five colors: {}", hexbot);
//!
//! let mut red_sum = 0;
//! for color in hexbot.colors() {
//!     red_sum += color.to_rgb255().0 as i32;
//! }
//! println!("The sum of all red values: {}", red_sum);
//!
//! let hexbot_with_coordinates = fetch_with_coordinates(5, 100, 100).unwrap();
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
use tint::Color;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct Dot {
    #[serde(rename = "value", deserialize_with = "deserialize_color")]
    color: Color,
    coordinates: Option<Coordinate>,
}

/// Deserialized response
#[derive(Clone, Debug, Deserialize, PartialEq)]
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
    /// let (x, y) = hexbot.coordinates().unwarp().get(1).unwrap();
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
    // #[inline]
    pub fn has_coordinates(&self) -> bool {
        !(self.colors[0].coordinates == None)
    }
}

impl fmt::Display for Hexbot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let len = self.colors.len() - 1;
        if self.has_coordinates() {
            for (counter, Dot { color, coordinates }) in self.colors.iter().enumerate() {
                if counter == len {
                    write!(
                        f,
                        "{}-({}|{})]",
                        color.to_hex().to_uppercase(),
                        coordinates.unwrap().x,
                        coordinates.unwrap().y
                    )?;
                } else {
                    write!(
                        f,
                        "{}-({}|{}), ",
                        color.to_hex().to_uppercase(),
                        coordinates.unwrap().x,
                        coordinates.unwrap().y
                    )?;
                }
            }
        } else {
            for (counter, Dot { color, .. }) in self.colors.iter().enumerate() {
                if counter == len {
                    write!(f, "{}]", color.to_hex().to_uppercase())?;
                } else {
                    write!(f, "{}, ", color.to_hex().to_uppercase())?;
                }
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
    if 1 <= count && count <= 1000 {
        Ok(reqwest::get(&format!(
            "https://api.noopschallenge.com/hexbot?count={}",
            count
        ))?
        .json()?)
    } else {
        Err(Error::CountOutOfRange)
    }
}

/// Fetch the given count of colors in the given range of coordinates from the Hexbot API.
///
/// ## Thresholds:
///  * `1 <= count <= 1,000`
///  * `10 <= width <= 100,000`
///  * `10 <= height <= 100,000`
pub fn fetch_with_coordinates(count: i32, width: i32, height: i32) -> Result<Hexbot, Error> {
    if 1 <= count && count <= 1000 {
        if 10 <= width && width <= 100_000 && 10 <= height && height <= 100_000 {
            Ok(reqwest::get(&format!(
                "https://api.noopschallenge.com/hexbot?count={}&width={}&height={}",
                count, width, height
            ))?
            .json()?)
        } else {
            Err(Error::WidthHeightOutOfRange)
        }
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
    /// Occurs when width and/or height is higher then 100,000 or lower then 10.
    WidthHeightOutOfRange,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Reqwest(ref err) => err.fmt(f),
            Error::CountOutOfRange => write!(f, "count must be between 1 and 1000"),
            Error::WidthHeightOutOfRange => {
                write!(f, "width and height must be between 10 and 100,000")
            }
        }
    }
}

impl error::Error for Error {
    // Usage of description is soft-deprecated; use Display
    #[cfg(ErrorDescription)]
    fn description(&self) -> &str {
        match self {
            Error::Reqwest(ref err) => err.description(),
            Error::CountOutOfRange => "count must be between 1 and 1000",
            Error::WidthHeightOutOfRange => "width and height must be between 10 and 100,000",
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Reqwest(ref err) => err.source(),
            Error::CountOutOfRange | Error::WidthHeightOutOfRange => None,
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
    fn test_fetch() {
        // check that 0 results in Error::CountOutOfRange
        assert!(match fetch(0) {
            Err(Error::CountOutOfRange) => true,
            _ => false,
        });
        // check that 1001 results in Error::CountOutOfRange
        assert!(match fetch(1001) {
            Err(Error::CountOutOfRange) => true,
            _ => false,
        });
        // check that 1 does not result in Error::CountOutOfRange
        assert!(match fetch(1) {
            Err(Error::CountOutOfRange) => false,
            _ => true,
        });
        // check that 1000 does not result in Error::CountOutOfRange
        assert!(match fetch(1000) {
            Err(Error::CountOutOfRange) => false,
            _ => true,
        });
        // Check that the Ok() data type of fetch is hexbot.
        // Note that this is not done during execution, it is done during compilation.
        #[allow(unused_variables)]
        match fetch(3) {
            Ok(hb) => {
                let hexbot: Hexbot = hb;
            }
            Err(err) => (),
        }
    }
}
