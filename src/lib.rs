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

#![warn(missing_docs)]
#![deny(missing_debug_implementations)]

// It's not needless.
#![allow(clippy::needless_doctest_main)]

//! Request the [Hexbot-API] and process the response.
//!
//! [Examples how api-request looks in hexbot.][1]
//!
//! # Examples
//!
//! Good: using `async`/`.await` with [tokio] and
//! error propagation using the question mark operator
//!
//! ```no_run
//! use hexbot::{Count, Hexbot, Seed, WidthHeight};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let hb = Hexbot::fetch(
//!         Count::yes(20)?,
//!         WidthHeight::yes(40, 60)?,
//!         &Seed::no()
//!     ).await?;
//!     println!("{}", hb);
//!     Ok(())
//! }
//! ```
//!
//! Bad: blocking with [futures], no `async`/`.await`;
//! error handling with `unwrap` and `expect`.
//!
//! Seems not to work anymore.
//!
//! ```no_run
//! use hexbot::{Count, Hexbot, Seed, WidthHeight};
//! use futures::executor::block_on;
//!
//! fn main() {
//!     let hb = block_on(Hexbot::fetch(
//!         Count::yes(20).unwrap(),
//!         WidthHeight::yes(40, 60).unwrap(),
//!         &Seed::no()
//!     )).expect("Fetching failed");
//!
//!     println!("{}", hb);
//! }
//! ```
//!
//! [1]: struct.Hexbot.html#examples-1
//! [Hexbot-API]: https://noopschallenge.com/challenges/hexbot
//! [tokio]: https://crates.io/crates/tokio
//! [futures]: https://crates.io/crates/futures

mod hexbot;

pub use tint::Color;
#[rustfmt::skip]
pub use crate::hexbot::{
    coordinates::Coordinates,
    count::Count,
    dot::Dot,
    errors,
    hexbot::Hexbot,
    seed::Seed,
    widthheight::WidthHeight,
};
