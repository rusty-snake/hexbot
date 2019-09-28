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

use crate::errors;
use std::{fmt, ops};

/// Representation of the `count` parameter of the [hexbot-API].
///
/// # Examples
///
/// ```no_run
/// # use hexbot::*;
/// // Fetch a Hexbot with 500 colors.
/// let hb = Hexbot::fetch(
///     Count::yes(500)?,
///     WithCoordinates::no(),
///     &Seed::no()
/// )?;
/// # assert_eq!(hb.len(), 500);
///
/// // Don't add the `count` parameter to the request.
/// let hb = Hexbot::fetch(
///     Count::no(),
///     WithCoordinates::no(),
///     &Seed::no()
/// )?;
/// # assert_eq!(hb.len(), 1);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// [hexbot-API]: https://github.com/noops-challenge/hexbot/blob/master/API.md
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub struct Count(Option<i32>);
impl Count {
    /// The minimum value for `count`.
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(Count::MIN, 1);
    /// ```
    pub const MIN: i32 = 1;

    /// The maximum value for `count`.
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(Count::MAX, 1000);
    /// ```
    pub const MAX: i32 = 1000;

    /// The allowed range for the value of `count`.
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(Count::ALLOWED_RANGE, (Count::MIN..=Count::MAX));
    /// ```
    pub const ALLOWED_RANGE: ops::RangeInclusive<i32> = (Self::MIN..=Self::MAX);

    /// Creates a new instance of `Count` with a value.
    ///
    /// # Errors
    ///
    /// [`CountOutOfRange`] occurs if `count` is not in [`ALLOWED_RANGE`].
    ///
    /// ## Examples
    ///
    /// ```should_panic
    /// # use hexbot::*;
    /// Count::yes(0).unwrap();
    /// ```
    /// ```should_panic
    /// # use hexbot::*;
    /// Count::yes(1001).unwrap();
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let count = Count::yes(30)?;
    /// # assert_eq!(count.get(), &Some(30));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`ALLOWED_RANGE`]: struct.Count.html#associatedconstant.ALLOWED_RANGE
    /// [`CountOutOfRange`]: errors/struct.CountOutOfRange.html
    pub fn yes(count: i32) -> Result<Self, errors::CountOutOfRange> {
        if Self::ALLOWED_RANGE.contains(&count) {
            Ok(Self(Some(count)))
        } else {
            Err(errors::CountOutOfRange)
        }
    }

    /// Creates a new instance of `Count` without a value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let no_count = Count::no();
    /// # assert_eq!(no_count.get(), &None);
    pub const fn no() -> Self {
        Self(None)
    }

    /// Creates a new instance of `Count` with the highest possible value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let max_count = Count::max();
    /// assert_eq!(max_count, Count::yes(Count::MAX)?);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub const fn max() -> Self {
        Self(Some(Self::MAX))
    }

    /// Creates a new instance of `Count` with the lowest possible value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let min_count = Count::min();
    /// assert_eq!(min_count, Count::yes(Count::MIN)?);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    pub const fn min() -> Self {
        Self(Some(Self::MIN))
    }

    /// Returns `true` if this `Count` has a value, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(Count::yes(20)?.is(), true);
    /// assert_eq!(Count::min().is(),    true);
    /// assert_eq!(Count::max().is(),    true);
    /// assert_eq!(Count::no().is(),    false);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is(&self) -> bool {
        self.0.is_some()
    }

    /// Returns a reference to the count or `None` if there is no.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(Count::yes(50)?.get(), &Some(50));
    /// assert_eq!(Count::min().get(), &Some(Count::MIN));
    /// assert_eq!(Count::max().get(), &Some(Count::MAX));
    /// assert_eq!(Seed::no().get(), &None);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get(&self) -> &Option<i32> {
        &self.0
    }
}
impl fmt::Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(count) => write!(f, "{}", count),
            None => write!(f, ""),
        }
    }
}
