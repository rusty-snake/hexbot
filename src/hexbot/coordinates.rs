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

use serde::Deserialize;
use std::{fmt, ops};

/// Generic representation for coordinates with an x and a y value.
///
/// It is used by [`Hexbot`] and [`CoordinatesLimit`], but can also be used anywhere else.
///
/// # Examples
///
/// ```no_run
/// # use hexbot::*;
/// let hb = Hexbot::fetch(
///     Count::yes(3)?,
///     WithCoordinates::yes(CoordinatesLimit::min()),
///     &Seed::no()
/// )
/// .expect("Fetching failed");
/// let coordinates = hb.dot_at(1).unwrap().coordinates.unwrap();
/// println!("You'll find the treasures at {}.", coordinates);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ```
/// # use hexbot::Coordinates;
/// let coordinates = Coordinates { x: 15, y: 30 };
/// if coordinates.x * 2 == coordinates.y {
///     println!("2*x = y");
/// } else {
///     println!("x is {}.", coordinates.x);
/// }
/// let Coordinates { x, y } = coordinates;
/// println!("{}!", x + y );
/// ```
///
/// ```
/// # use hexbot::Coordinates;
/// let coordinates1 = Coordinates { x: 15, y: 30 };
/// let coordinates2 = Coordinates { x: 15, y: 30 };
/// assert_eq!(
///     coordinates1 + coordinates2,
///     Coordinates { x: 30, y: 60 }
/// );
/// assert_eq!(
///     coordinates1 - coordinates2,
///     Coordinates { x: 0, y: 0 }
/// );
/// ```
///
/// [`Hexbot`]: struct.Hexbot.html
/// [`CoordinatesLimit`]: struct.CoordinatesLimit.html
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}
impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}|{})", self.x, self.y)
    }
}
impl ops::Add for Coordinates {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl ops::Sub for Coordinates {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
