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

use crate::{Count, Dot, Seed, __WidthHeight};
use serde::Deserialize;
use std::{
    fmt::{self, Write},
    slice, vec,
};
use tint::Color;

const API_ENDPOINT: &str = "https://api.noopschallenge.com/hexbot?";

/// Abstract representation of the response from the hexbot API.
///
/// # Examples
///
/// ```no_run
/// # use hexbot::*;
/// let hb = Hexbot::fetch(
///     Count::yes(50)?,
///     WithCoordinates::no(),
///     &Seed::no()
/// )?;
///
/// println!("The first color: {}.", hb.color_at(0).unwrap().to_hex());
/// println!("The last color: {}.", hb.color_at(hb.len() - 1).unwrap().to_hex());
///
/// if !hb.has_coordinates() {
///     println!("Ohh no you don't know where to go!! :(");
/// }
///
/// let Dot { color: color25, .. } = hb.dot_at(24).unwrap();
/// println!("The red value from the color at position 25: {}", color25.red);
///
/// for dot in &hb {
///     println!("{}", dot.color.to_hex());
/// }
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Hexbot {
    colors: Vec<Dot>,
}
#[allow(clippy::len_without_is_empty)] // Hexbot.colors should never be empty
impl Hexbot {
    /// Creates a new instance of `Hexbot`
    ///
    /// `coordinates` can be [`WidthHeight`] or [`WithCoordinates`], but you should use
    /// [`WidthHeight`].
    ///
    /// # Errors
    ///
    /// <https://docs.rs/reqwest/0.9.20/reqwest/struct.Error.html>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use hexbot::*;
    /// // https://api.noopschallenge.com/hexbot?
    /// let hb1 = Hexbot::fetch(
    ///     Count::no(),
    ///     WithCoordinates::no(),
    ///     &Seed::no()
    /// )?;
    /// // https://api.noopschallenge.com/hexbot?count=100
    /// let hb2 = Hexbot::fetch(
    ///     Count::yes(100)?,
    ///     WithCoordinates::no(),
    ///     &Seed::no()
    /// )?;
    /// // https://api.noopschallenge.com/hexbot?width=40&height=60
    /// let hb3 = Hexbot::fetch(
    ///     Count::no(),
    ///     WithCoordinates::yes(CoordinatesLimit::new(40, 60)?),
    ///     &Seed::no()
    /// )?;
    /// // https://api.noopschallenge.com/hexbot?seed=B7410E,B22222
    /// let hb4 = Hexbot::fetch(
    ///     Count::no(),
    ///     WithCoordinates::no(),
    ///     &Seed::new(&[0x_B7_41_0E, 0x_B2_22_22])?
    /// )?;
    /// // https://api.noopschallenge.com/hexbot?count=70&width=400&height=400
    /// let hb5 = Hexbot::fetch(
    ///     Count::yes(70)?,
    ///     WithCoordinates::yes(CoordinatesLimit::new(400, 400)?),
    ///     &Seed::no()
    /// )?;
    /// // https://api.noopschallenge.com/hexbot?count=1000&width=10&height=10
    /// let hb6 = Hexbot::fetch(
    ///     Count::max(),
    ///     WithCoordinates::yes(CoordinatesLimit::min()),
    ///     &Seed::no()
    /// )?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`WidthHeight`]: struct.WidthHeight.html
    /// [`WithCoordinates`]: struct.WithCoordinates.html
    pub fn fetch(
        count: Count,
        coordinates: impl __WidthHeight,
        seed: &Seed,
    ) -> Result<Self, reqwest::Error> {
        let count = match count.get() {
            None => String::new(),
            Some(count) => format!("&count={}", count),
        };
        let coordinates = match __WidthHeight::get(&coordinates) {
            None => String::new(),
            Some(coordinates) => format!("&width={}&height={}", coordinates.x, coordinates.y),
        };
        let seed = match seed.get() {
            None => String::new(),
            Some(seed) => format!("&seed={}", seed),
        };
        reqwest::get(&format!("{}{}{}{}", API_ENDPOINT, count, coordinates, seed))?.json()
    }

    /// Returns a reference to a [`Color`] or `None` if out of bounds.
    ///
    /// ```no_run
    /// # use hexbot::*;
    /// let hb = Hexbot::fetch(
    ///     Count::yes(3)?,
    ///     WithCoordinates::no(),
    ///     &Seed::new(&[0x_FF_FF_FF])?
    /// )
    /// .expect("Fetching failed");
    ///
    /// println!("The first color is {}.", hb.color_at(0).unwrap().to_hex());
    ///
    /// assert_eq!(hb.color_at(2), Some(&Color::from("#FFFFFF")));
    /// assert_eq!(hb.color_at(3), None);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`Color`]: struct.Color.html
    pub fn color_at(&self, index: usize) -> Option<&Color> {
        Some(&self.colors.get(index)?.color)
    }

    /// Returns a reference to a [`Dot`] or `None` if out of bounds.
    ///
    /// ```no_run
    /// # use hexbot::*;
    /// let hb = Hexbot::fetch(
    ///     Count::no(),
    ///     WithCoordinates::no(),
    ///     &Seed::new(&[0x_AB_CD_EF])?
    /// )
    /// .expect("Fetching failed");
    ///
    /// assert_eq!(
    ///     hb.dot_at(0),
    ///     Some(&Dot { color: Color::from("#ABCDEF"), coordinates: None }),
    /// );
    /// assert_eq!(hb.dot_at(1), None);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`Dot`]: struct.Dot.html
    pub fn dot_at(&self, index: usize) -> Option<&Dot> {
        self.colors.get(index)
    }

    /// Returns `true` if this Hexbot has coordinates, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use hexbot::*;
    /// let hexbot_with_coordinates = Hexbot::fetch(
    ///     Count::yes(6)?,
    ///     WithCoordinates::yes(CoordinatesLimit::new(20, 20)?),
    ///     &Seed::no()
    /// )
    /// .expect("Fetching failed");
    /// assert_eq!(hexbot_with_coordinates.has_coordinates(), true);
    ///
    /// let hexbot_without_coordinates = Hexbot::fetch(
    ///      Count::yes(6)?,
    ///      WithCoordinates::no(),
    ///      &Seed::no()
    /// )
    /// .expect("Fetching failed");
    /// assert_eq!(hexbot_without_coordinates.has_coordinates(), false);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn has_coordinates(&self) -> bool {
        !(self.colors[0].coordinates == None)
    }

    /// Returns the number of colors in this hexbot.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use hexbot::*;
    /// let hb = Hexbot::fetch(
    ///     Count::yes(5)?,
    ///     WithCoordinates::no(),
    ///     &Seed::no()
    /// )
    /// .expect("Fetching failed");
    /// assert_eq!(hb.len(), 5);
    ///
    /// let hb = Hexbot::fetch(
    ///     Count::no(),
    ///     WithCoordinates::yes(CoordinatesLimit::new(500, 500)?),
    ///     &Seed::no()
    /// )
    /// .expect("Fetching failed");
    /// assert_eq!(hb.len(), 1);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// Consumes the `Hexbot` and return its inner `Vec`.
    ///
    /// ```no_run
    /// # use hexbot::*;
    /// let hb = Hexbot::fetch(
    ///     Count::yes(5)?,
    ///     WithCoordinates::no(),
    ///     &Seed::new(&[0x_00_AA_00])?
    /// )
    /// .expect("Fetching failed");
    /// let dots = hb.into_inner();
    /// assert_eq!(
    ///     dots,
    ///     vec![Dot { color: Color::from("#00AA00"), coordinates: None }; 5],
    /// );
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// ```compile_fail
    /// # use hexbot::*;
    /// let hb = Hexbot::fetch(
    ///     Count::yes(5)?,
    ///     WithCoordinates::no(),
    ///     &Seed::new(&[0x_00_AA_00])?
    /// )
    /// .expect("Fetching failed");
    /// let dots = hb.into_inner();
    /// println!("{}", hb);
    /// println!("{:?}", dots);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_inner(self) -> Vec<Dot> {
        self.colors
    }

    /// Returns a reference to the inner `Vec` of this Hexbot.
    ///
    /// ```no_run
    /// # use hexbot::*;
    /// let hexbot = Hexbot::fetch(
    ///     Count::yes(5)?,
    ///     WithCoordinates::no(),
    ///     &Seed::new(&[0x_00_00_FF])?
    /// )
    /// .expect("Fetching failed");
    /// assert_eq!(
    ///     hexbot.as_inner(),
    ///     &vec![Dot { color: Color::from("#0000FF"), coordinates: None }; 5],
    /// );
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn as_inner(&self) -> &Vec<Dot> {
        &self.colors
    }

    #[doc(hidden)]
    pub fn iter(&self) -> slice::Iter<'_, Dot> {
        self.colors.iter()
    }
}
impl fmt::Display for Hexbot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for dot in &self.colors {
            write!(s, "{}, ", dot)?;
        }
        write!(f, "[{}]", &s[..s.len() - 2])
    }
}
impl IntoIterator for Hexbot {
    type Item = Dot;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.colors.into_iter()
    }
}
impl<'a> IntoIterator for &'a Hexbot {
    type Item = &'a Dot;
    type IntoIter = slice::Iter<'a, Dot>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
