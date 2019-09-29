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

use crate::errors::SeedError;
use std::fmt::{self, Write};

/// Representation of the `seed` parameter of the [hexbot-API].
///
/// # Examples
///
/// ```no_run
/// # use hexbot::*;
/// // Fetch a Hexbot with this seed: `00FFFF,0000FF,008B8B,00008B`.
/// let hb = Hexbot::fetch(
///     Count::no(),
///     WithCoordinates::no(),
///     &Seed::new(&[0x_00_FF_FF, 0x_00_00_FF, 0x_00_8B_8B, 0x_00_00_8B])?
/// )?;
///
/// // Don't add the `seed` parameter to the request.
/// let hb = Hexbot::fetch(
///     Count::no(),
///     WithCoordinates::no(),
///     &Seed::no()
/// )?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// [hexbot-API]: https://github.com/noops-challenge/hexbot/blob/master/API.md
#[derive(Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct Seed(Option<String>);
impl Seed {
    /// Creates a new instance of `Seed` with a seed.
    ///
    /// # Errors
    ///
    /// The error type [`SeedError`] has three types:
    ///  - [`SeedError::Empty`] occurs if `colors` is an empty slice.
    ///  - [`SeedError::ToLong`] occurs if `colors` has 11 or more elements.
    ///  - [`SeedError::NoColor`] occurs if an element in `colors` isn't a valid color.
    ///    A valid color in a number between 0 (`0x_00_00_00`) and 16777215 (`0x_FF_FF_FF`).  
    ///    **This variant will change in the future to `NoColor(i32)` which contains the bad
    ///    element.**
    ///
    /// ## Examples
    ///
    /// ```should_panic
    /// # use hexbot::*;
    /// Seed::new(&[]).unwrap();
    /// ```
    /// ```should_panic
    /// # use hexbot::*;
    /// Seed::new(&[0x_AA_AA_AA; 11]).unwrap();
    /// ```
    /// ```should_panic
    /// # use hexbot::*;
    /// Seed::new(&[-1]).unwrap();
    /// ```
    /// ```should_panic
    /// # use hexbot::*;
    /// Seed::new(&[0x_FF_FF_FF + 1]).unwrap();
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let seed = Seed::new(&[0x_8B_00_00, 0x_8B_00_8B])?;
    /// # assert_eq!(*seed.get(), Some(String::from("8B0000,8B008B")));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`SeedError`]: errors/enum.SeedError.html
    /// [`SeedError::Empty`]: errors/enum.SeedError.html#variant.Empty
    /// [`SeedError::ToLong`]: errors/enum.SeedError.html#variant.ToLong
    /// [`SeedError::NoColor`]: errors/enum.SeedError.html#variant.NoColor
    pub fn new(colors: &[i32]) -> Result<Self, SeedError> {
        if colors.is_empty() {
            return Err(SeedError::Empty);
        }
        if colors.len() > 10 {
            return Err(SeedError::ToLong);
        }
        let mut seed = String::with_capacity(colors.len() * 7);
        for color in colors {
            if 0x_00_00_00 <= *color && *color <= 0x_FF_FF_FF {
                write!(&mut seed, "{:06X},", color).unwrap();
            } else {
                return Err(SeedError::NoColor);
            }
        }
        seed.pop();
        seed.shrink_to_fit();
        Ok(Self(Some(seed)))
    }

    /// Creates a new instance of `Seed` without seed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let no_seed = Seed::no();
    /// # assert_eq!(no_seed.get(), &None);
    /// ```
    pub const fn no() -> Self {
        Self(None)
    }

    /// Returns `true` if this `Seed` has a seed, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(Seed::new(&[0x_00_00_00, 0x_99_99_99])?.is(), true);
    /// assert_eq!(Seed::no().is(), false);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is(&self) -> bool {
        self.0.is_some()
    }

    /// Returns a reference to the seed or `None` if there is no seed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// assert_eq!(
    ///     Seed::new(&[0x_11_11_00, 0x_FF_FF_00, 0x_FF_A5_00])?.get(),
    ///     &Some(String::from("111100,FFFF00,FFA500"))
    /// );
    /// assert_eq!(Seed::no().get(), &None);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get(&self) -> &Option<String> {
        &self.0
    }

    /// Add a color to an existing `Seed`.
    ///
    /// # Errors
    ///
    /// [`SeedError`] (same as [`new`]):
    ///  - [`SeedError::ToLong`] occurs if the seed already has 10 colors.
    ///  - [`SeedError::NoColor`] occurs if `color` isn't a valid color.
    ///    A valid color in a number between 0 (`0x_00_00_00`) and 16777215 (`0x_FF_FF_FF`).  
    ///    **This variant will change in the future to `NoColor(i32)`.**
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexbot::*;
    /// let mut seed = Seed::new(&[0x_11_11_00, 0x_FF_FF_00, 0x_FF_A5_00])?;
    /// seed.add(0x_AA_00_AA)?;
    /// assert_eq!(
    ///     seed.get(),
    ///     &Some(String::from("111100,FFFF00,FFA500,AA00AA"))
    /// );
    ///
    /// let mut seed = Seed::no();
    /// seed.add(0x_AA_00_AA)?;
    /// seed.add(0x_AB_CD_EF)?;
    /// assert_eq!(
    ///     seed.get(),
    ///     &Some(String::from("AA00AA,ABCDEF"))
    /// );
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// ```compile_fail
    /// # use hexbot::*;
    /// // seed must be mutable
    /// let seed = Seed::new(&[0x_22_22_00, 0x_99_99_00])?;
    /// seed.add(0x_01_23_45)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`new`]: #method.new
    /// [`SeedError`]: errors/enum.SeedError.html
    /// [`SeedError::ToLong`]: errors/enum.SeedError.html#variant.ToLong
    /// [`SeedError::NoColor`]: errors/enum.SeedError.html#variant.NoColor
    pub fn add(&mut self, color: i32) -> Result<(), SeedError> {
        if color < 0x_00_00_00 || color > 0x_FF_FF_FF {
            return Err(SeedError::NoColor);
        }
        match self.0 {
            Some(ref mut seed) => write!(seed, ",{:06X}", color).unwrap(),
            None => self.0 = Some(Self::new(&[color])?.0.unwrap()),
        };
        Ok(())
    }
}
impl fmt::Display for Seed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(seed) => write!(f, "{}", seed),
            None => write!(f, ""),
        }
    }
}
