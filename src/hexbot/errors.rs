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

//! All custom Error types used by hexbot.

use std::{error::Error as StdError, fmt};

/// Error type from [`Count::yes()`].
///
/// [`Count::yes()`]: ../struct.Count.html#method.yes
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CountOutOfRange;
impl StdError for CountOutOfRange {}
impl fmt::Display for CountOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The given count was out of range.")
    }
}

/// Error type from [`CoordinatesLimit::new()`].
///
/// [`CoordinatesLimit::new()`]: ../struct.CoordinatesLimit.html#method.new
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CoordinatesLimitOutOfRange;
impl StdError for CoordinatesLimitOutOfRange {}
impl fmt::Display for CoordinatesLimitOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A given limit was out of range.")
    }
}

/// Error type from [`Seed::new()`].
///
/// Description for the different error variants can be found [here].
///
/// [here]: ../struct.Seed.html#errors
/// [`Seed::new()`]: ../struct.Seed.html#method.new
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SeedError {
    Empty,
    ToLong,
    /// **This variant will change in the future to `NoColor(i32)`.**
    NoColor,
}
impl StdError for SeedError {}
impl fmt::Display for SeedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "The given seed was an empty slice."),
            Self::ToLong => write!(f, "The given seed had 11 or more colors."),
            Self::NoColor => write!(f, "A given color wasn't a color."),
        }
    }
}
