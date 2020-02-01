/*
 * Copyright © 2019,2020 rusty-snake <print_hello_world+License@protonmail.com>
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

use crate::{errors::CoordinatesLimitOutOfRange, Coordinates};
use std::{convert, fmt, ops};

/// Representation of the `width` and `height` parameters of the [hexbot-API].
///
/// # Examples
///
/// ```no_run
/// # use hexbot::*;
/// # async {
/// let hb_with_coordinates = Hexbot::fetch(
///     Count::no(),
///     WidthHeight::yes(10_000, 2500)?,
///     &Seed::no()
/// ).await?;
/// let hb_without_coordinates = Hexbot::fetch(
///     Count::no(),
///     WidthHeight::no(),
///     &Seed::no()
/// ).await?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// # };
/// ```
///
/// ```
/// # use hexbot::*;
/// use std::convert::TryFrom;
/// let width_height = WidthHeight::try_from(
///     Coordinates { x: 20, y: 20 }
/// )?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// [hexbot-API]: https://github.com/noops-challenge/hexbot/blob/master/API.md
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub struct WidthHeight(Option<Coordinates>);
impl WidthHeight {
    /// The minimum value for `width`/`height`.
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(WidthHeight::MIN, 10);
    /// ```
    pub const MIN: i32 = 10;

    /// The maximum value for `width`/`height`.
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(WidthHeight::MAX, 100_000);
    /// ```
    pub const MAX: i32 = 100_000;

    /// The allowed range for the value of `width`/`height`.
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(
    ///     WidthHeight::ALLOWED_RANGE,
    ///     (WidthHeight::MIN..=WidthHeight::MAX)
    /// );
    /// ```
    pub const ALLOWED_RANGE: ops::RangeInclusive<i32> = (Self::MIN..=Self::MAX);

    /// Creates a new instance of `WidthHeight`.
    ///
    /// # Errors
    ///
    /// [`CoordinatesLimitOutOfRange`] occurs if `x` and/or `y` is not in [`ALLOWED_RANGE`].
    ///
    /// ## Examples
    ///
    /// ```should_panic
    /// # use hexbot::*;
    /// WidthHeight::yes(9, 40).unwrap();
    /// ```
    /// ```should_panic
    /// # use hexbot::*;
    /// WidthHeight::yes(100_001, 40).unwrap();
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let width_height = WidthHeight::yes(50, 80)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`ALLOWED_RANGE`]: #associatedconstant.ALLOWED_RANGE
    /// [`CoordinatesLimitOutOfRange`]: errors/struct.CoordinatesLimitOutOfRange.html
    pub fn yes(width: i32, height: i32) -> Result<Self, CoordinatesLimitOutOfRange> {
        if Self::ALLOWED_RANGE.contains(&width) && Self::ALLOWED_RANGE.contains(&height) {
            Ok(Self(Some(Coordinates {
                x: width,
                y: height,
            })))
        } else {
            Err(CoordinatesLimitOutOfRange)
        }
    }

    /// Creates a new empty instance of `WidthHeight`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let width_height = WidthHeight::no();
    /// ```
    pub const fn no() -> Self {
        Self(None)
    }

    /// Creates a new instance of `WidthHeight` with the highest possible values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let max_width_height = WidthHeight::max();
    /// assert_eq!(
    ///     max_width_height,
    ///     WidthHeight::yes(WidthHeight::MAX, WidthHeight::MAX)?
    /// );
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub const fn max() -> Self {
        Self(Some(Coordinates {
            x: Self::MAX,
            y: Self::MAX,
        }))
    }

    /// Creates a new instance of `WidthHeight` with the lowest possible values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let min_width_height = WidthHeight::min();
    /// assert_eq!(
    ///     min_width_height,
    ///     WidthHeight::yes(WidthHeight::MIN, WidthHeight::MIN)?
    /// );
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub const fn min() -> Self {
        Self(Some(Coordinates {
            x: Self::MIN,
            y: Self::MIN,
        }))
    }

    /// Returns `true` if this `WidthHeight` isn't empty, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(
    ///     WidthHeight::yes(1024,1024)?.has(),
    ///     true
    /// );
    /// assert_eq!(WidthHeight::no().has(), false);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn has(&self) -> bool {
        self.0.is_some()
    }

    /// Returns a reference to the values of width and height.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(
    ///     WidthHeight::yes(25, 25)?.get(),
    ///     &Some(Coordinates { x: 25, y: 25 })
    /// );
    /// assert_eq!(
    ///     WidthHeight::min().get(),
    ///     &Some(Coordinates {
    ///         x: WidthHeight::MIN,
    ///         y: WidthHeight::MIN,
    ///     })
    /// );
    /// assert_eq!(
    ///     WidthHeight::max().get(),
    ///     &Some(Coordinates {
    ///         x: WidthHeight::MAX,
    ///         y: WidthHeight::MAX,
    ///     })
    /// );
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get(&self) -> &Option<Coordinates> {
        &self.0
    }
}
impl fmt::Display for WidthHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(ref value) => write!(f, "width:{},height:{}", value.x, value.y),
            None => write!(f, ""),
        }
    }
}
impl convert::TryFrom<Coordinates> for WidthHeight {
    type Error = CoordinatesLimitOutOfRange;

    fn try_from(coordinates: Coordinates) -> Result<Self, Self::Error> {
        Self::yes(coordinates.x, coordinates.y)
    }
}
