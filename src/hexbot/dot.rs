/*
 * Copyright © 2019 rusty-snake <print_hello_world+License@protonmail.com>
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

use crate::Coordinates;
use serde::{Deserialize, Deserializer};
use std::fmt;
use tint::Color;

/// Representation of an entry in the colors array [[1][API-doc]].
///
/// # Examples
///
/// ```no_run
/// # use hexbot::*;
/// let hb1 = Hexbot::fetch(
///     Count::yes(5)?,
///     WithCoordinates::no(),
///     &Seed::new(&[0x_B7_41_0E])?
/// )
/// .expect("Fetching failed");
/// let dot = hb1.dot_at(2).unwrap();
/// assert_eq!(dot, &Dot { color: Color::from("#B7410E"), coordinates: None });
///
/// let hb2 = Hexbot::fetch(
///     Count::yes(5)?,
///     WithCoordinates::yes(CoordinatesLimit::new(2500, 4000)?),
///     &Seed::no()
/// )
/// .expect("Fetching failed");
/// let Dot { color, coordinates } = hb2.dot_at(4).unwrap();
/// println!("color: {}, coordinates: {}", color, coordinates.unwrap());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// [API-doc]: https://github.com/noops-challenge/hexbot/blob/master/API.md
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct Dot {
    #[serde(rename = "value", deserialize_with = "deserialize_color")]
    pub color: Color,
    pub coordinates: Option<Coordinates>,
}
impl Dot {
    /// Returns `true` if this Dot has coordinates, otherwise `false`.
    ///
    /// ```no_run
    /// # use hexbot::*;
    /// let hexbot_without_coordinates = Hexbot::fetch(
    ///     Count::no(),
    ///     WithCoordinates::no(),
    ///     &Seed::no()
    /// )
    /// .expect("Fetching failed");
    /// let dot_without_coordinates = hexbot_without_coordinates.dot_at(0).unwrap();
    /// assert_eq!(dot_without_coordinates.has_coordinates(), false);
    ///
    /// let hexbot_with_coordinates = Hexbot::fetch(
    ///     Count::no(),
    ///     WithCoordinates::yes(CoordinatesLimit::new(20, 20)?),
    ///     &Seed::no()
    /// )
    /// .expect("Fetching failed");
    /// let dot_with_coordinates = hexbot_with_coordinates.dot_at(0).unwrap();
    /// assert_eq!(dot_with_coordinates.has_coordinates(), true);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn has_coordinates(&self) -> bool {
        !(self.coordinates == None)
    }
}
impl fmt::Display for Dot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.color.to_hex().to_uppercase())?;
        if let Some(coordinates) = self.coordinates {
            write!(f, "-{}", coordinates)?;
        }
        Ok(())
    }
}

fn deserialize_color<'de, D: Deserializer<'de>>(deser: D) -> Result<Color, D::Error> {
    Ok(Color::from_hex(&String::deserialize(deser)?))
}
