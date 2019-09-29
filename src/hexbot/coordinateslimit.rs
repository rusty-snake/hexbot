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

#![deprecated(since = "0.0.11", note = "Use WidthHeight instead.")]

use crate::{errors::CoordinatesLimitOutOfRange, Coordinates};
use std::{fmt, ops};

/// Concrete representation of the `width` and the `height` parameters of the [hexbot-API].
///
/// # Examples
///
/// ```no_run
/// # use hexbot::*;
/// // Fetch a Hexbot with coordinates.
/// // The x coordinate has a maximum of 10,000
/// // and the y coordinate of 2,5000.
/// let hb = Hexbot::fetch(
///     Count::no(),
///     WithCoordinates::yes(CoordinatesLimit::new(10_000, 2500)?),
///     &Seed::no()
/// )?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// [hexbot-API]: https://github.com/noops-challenge/hexbot/blob/master/API.md
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CoordinatesLimit(Coordinates);
impl CoordinatesLimit {
    /// The minimum value for `width`/`height`.
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(CoordinatesLimit::MIN, 10);
    /// ```
    pub const MIN: i32 = 10;

    /// The maximum value for `width`/`height`.
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(CoordinatesLimit::MAX, 100_000);
    /// ```
    pub const MAX: i32 = 100_000;

    /// The allowed range for the value of `width`/`height`.
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(
    ///     CoordinatesLimit::ALLOWED_RANGE,
    ///     (CoordinatesLimit::MIN..=CoordinatesLimit::MAX)
    /// );
    /// ```
    pub const ALLOWED_RANGE: ops::RangeInclusive<i32> = (Self::MIN..=Self::MAX);

    /// Creates a new instance of `CoordinatesLimit`.
    ///
    /// # Errors
    ///
    /// [`CoordinatesLimitOutOfRange`] occurs if `x` and/or `y` is not in [`ALLOWED_RANGE`].
    ///
    /// ## Examples
    ///
    /// ```should_panic
    /// # use hexbot::*;
    /// CoordinatesLimit::new(9, 40).unwrap();
    /// ```
    /// ```should_panic
    /// # use hexbot::*;
    /// CoordinatesLimit::new(100_001, 40).unwrap();
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let coordinates_limit = CoordinatesLimit::new(50, 30)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`ALLOWED_RANGE`]: struct.CoordinatesLimit.html#associatedconstant.ALLOWED_RANGE
    /// [`CoordinatesLimitOutOfRange`]: errors/struct.CoordinatesLimitOutOfRange.html
    pub fn new(x: i32, y: i32) -> Result<Self, CoordinatesLimitOutOfRange> {
        if Self::ALLOWED_RANGE.contains(&x) && Self::ALLOWED_RANGE.contains(&y) {
            Ok(Self(Coordinates { x, y }))
        } else {
            Err(CoordinatesLimitOutOfRange)
        }
    }

    /// Creates a new instance of `CoordinatesLimit` with the highest possible limit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let max_coordinates_limit = CoordinatesLimit::max();
    /// assert_eq!(
    ///     max_coordinates_limit,
    ///     CoordinatesLimit::new(CoordinatesLimit::MAX, CoordinatesLimit::MAX)?
    /// );
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub const fn max() -> Self {
        Self(Coordinates {
            x: Self::MAX,
            y: Self::MAX,
        })
    }

    /// Creates a new instance of `CoordinatesLimit` with the lowest possible limit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let min_coordinates_limit = CoordinatesLimit::min();
    /// assert_eq!(
    ///     min_coordinates_limit,
    ///     CoordinatesLimit::new(CoordinatesLimit::MIN, CoordinatesLimit::MIN)?
    /// );
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub const fn min() -> Self {
        Self(Coordinates {
            x: Self::MIN,
            y: Self::MIN,
        })
    }

    /// Returns a reference to the values of the limit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(
    ///     CoordinatesLimit::new(25, 25)?.get(),
    ///     &Coordinates { x: 25, y: 25 }
    /// );
    /// assert_eq!(
    ///     CoordinatesLimit::min().get(),
    ///     &Coordinates {
    ///         x: CoordinatesLimit::MIN,
    ///         y: CoordinatesLimit::MIN,
    ///     }
    /// );
    /// assert_eq!(
    ///     CoordinatesLimit::max().get(),
    ///     &Coordinates {
    ///         x: CoordinatesLimit::MAX,
    ///         y: CoordinatesLimit::MAX,
    ///     }
    /// );
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get(&self) -> &Coordinates {
        &self.0
    }
}
impl fmt::Display for CoordinatesLimit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}
