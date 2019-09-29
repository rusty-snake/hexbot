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

#![warn(missing_docs)]
#![deny(missing_debug_implementations)]

//! Request the [Hexbot-API] and process the response.
//!
//! [Examples how api-request looks in hexbot.][1]
//!
//! # Examples
//!
//! ```no_run
//! use hexbot::{CoordinatesLimit, Count, Hexbot, Seed, WithCoordinates};
//! use std::io;
//! use std::io::Write;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     println!("===== Hexbot =====");
//!     let count = if ask_bool("Should the count parameter be added? [yes|no] ")? {
//!         loop {
//!             match Count::yes(ask_i32("What value should count have? [1-1000] ")?) {
//!                 Ok(count) => break count,
//!                 Err(_) => println!("This is a invalid count, try again."),
//!             }
//!         }
//!     } else {
//!         Count::no()
//!     };
//!     let with_coordinates =
//!         if ask_bool("Should the width and height parameters be added? [yes|no] ")? {
//!             WithCoordinates::yes(loop {
//!                 if let Ok(limit) = CoordinatesLimit::new(
//!                     ask_i32("What value should width have? [10-100,000] ")?,
//!                     ask_i32("What value should height have? [10-100,000] ")?,
//!                 ) {
//!                     break limit;
//!                  } else {
//!                     println!("This is a invalid width/height, try again.");
//!                 }
//!             })
//!         } else {
//!             WithCoordinates::no()
//!         };
//!     let hb = Hexbot::fetch(count, with_coordinates, &Seed::no()).expect("Fetching failed");
//!     println!("{}", hb);
//!     Ok(())
//! }
//!
//! fn input(prompt: &str) -> io::Result<String> {
//!     let mut buffer = String::new();
//!     let mut stdout = io::stdout();
//!     let stdin = io::stdin();
//!     stdout.write_all(prompt.as_bytes())?;
//!     stdout.flush()?;
//!     stdin.read_line(&mut buffer)?;
//!     Ok(buffer.trim().to_string())
//! }
//!
//! fn ask_bool(question: &str) -> io::Result<bool> {
//!     let answer = input(question)?.to_lowercase();
//!     if answer == "y" || answer == "yes" {
//!         Ok(true)
//!     } else if answer == "n" || answer == "no" {
//!         Ok(false)
//!     } else {
//!         println!("Sorry, try again.");
//!         ask_bool(question)
//!     }
//! }
//!
//! fn ask_i32(question: &str) -> io::Result<i32> {
//!     if let Ok(answer) = input(question)?.parse() {
//!         Ok(answer)
//!     } else {
//!         println!("Sorry, try again.");
//!         ask_i32(question)
//!     }
//! }
//! ```
//!
//! [1]: struct.Hexbot.html#examples-1
//! [Hexbot-API]: https://noopschallenge.com/challenges/hexbot

mod hexbot;

pub use tint::Color;
#[rustfmt::skip]
// Don't warn about CoordinatesLimit and WithCoordinates
#[allow(deprecated)]
pub use crate::hexbot::{
    coordinates::Coordinates,
    coordinateslimit::CoordinatesLimit,
    count::Count,
    dot::Dot,
    errors,
    hexbot::Hexbot,
    seed::Seed,
    withcoordinates::WithCoordinates,
    widthheight::WidthHeight,
};

#[doc(hidden)]
pub trait __WidthHeight: private::Sealed {
    fn get(&self) -> Option<Coordinates>;
}

#[allow(deprecated)]
mod private {
    pub trait Sealed {}
    impl Sealed for crate::WidthHeight {}
    impl Sealed for crate::WithCoordinates {}
}
