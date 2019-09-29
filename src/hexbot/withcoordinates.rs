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
// Don't warn about CoordinatesLimit
#![allow(deprecated)]

use crate::{Coordinates, CoordinatesLimit};
use std::fmt;

/// Abstract representation of the `width` and the `height` parameter of the [hexbot-API].
///
/// # Examples
///
/// ```no_run
/// # use hexbot::*;
/// // Fetch a Hexbot with coordinates up to 10,000.
/// let hb = Hexbot::fetch(
///     Count::no(),
///     WithCoordinates::yes(CoordinatesLimit::new(10_000, 10_000)?),
///     &Seed::no()
/// )?;
///
/// // Don't add the `width` and `height` parameters to the request.
/// let hb = Hexbot::fetch(
///     Count::no(),
///     WithCoordinates::no(),
///     &Seed::no()
/// )?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// [hexbot-API]: https://github.com/noops-challenge/hexbot/blob/master/API.md
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub struct WithCoordinates(Option<CoordinatesLimit>);
impl WithCoordinates {
    /// Creates a new instance of `WithCoordinates` with a `CoordinatesLimit`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let with_coordinates = WithCoordinates::yes(
    ///     CoordinatesLimit::new(50, 80)?
    /// );
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn yes(limit: CoordinatesLimit) -> Self {
        Self(Some(limit))
    }

    /// Creates a new instance of `WithCoordinates` without a limit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let with_coordinates = WithCoordinates::no();
    /// ```
    pub const fn no() -> Self {
        Self(None)
    }

    /// Returns `true` if this `WithCoordinates` has a `CoordinatesLimit`, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(
    ///     WithCoordinates::yes(CoordinatesLimit::new(1024,1024)?).with(),
    ///     true
    /// );
    /// assert_eq!(WithCoordinates::no().with(), false);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn with(&self) -> bool {
        self.0.is_some()
    }

    /// Returns a reference to the limit or `None` if there is no.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(
    ///     WithCoordinates::yes(CoordinatesLimit::new(1024,1024)?).limit(),
    ///     &Some(CoordinatesLimit::new(1024, 1024)?)
    /// );
    /// assert_eq!(WithCoordinates::no().limit(), &None);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn limit(&self) -> &Option<CoordinatesLimit> {
        &self.0
    }
}
impl fmt::Display for WithCoordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(limit) => write!(f, "{}", limit),
            None => write!(f, ""),
        }
    }
}
impl crate::__WidthHeight for WithCoordinates {
    fn get(&self) -> Option<Coordinates> {
        Some(*(*self.limit())?.get())
    }
}
