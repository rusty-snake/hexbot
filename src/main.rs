/*
 * Copyright (C) 2019 rusty-snake <print_hello_world+License@protonmail.com>
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

// NOTE: needs to be `pub` so that the documentation will be created.
pub mod request_api;

use request_api::{fetch, fetch_with_coordinates, Error};

fn main() -> Result<(), Error>{
    //
    // New hexbot with the parameter `count=20`.
    //
    let hexbot = fetch(20, None)?;
    println!("A hexbot with twenty colors: {}", hexbot);

    // sum all red values

    let mut red_sum = 0;
    for color in hexbot.colors() {
        red_sum += i32::from(color.to_rgb255().0);
    }
    println!("The sum of all red values: {}", red_sum);

    //
    // New hexbot with the parameters `count=5&width=100&heigth=100`.
    //
    let hexbot_with_coordinates = fetch_with_coordinates(5, 100, 100, None)?;
    println!(
        "A hexbot with five colors and coordiantes: {}",
        hexbot_with_coordinates
    );

    // Show the blue component of the second color and print its position.

    let coordinate = hexbot_with_coordinates.coordinates().unwrap()[1];
    let color = hexbot_with_coordinates.colors()[1];
    println!(
        "The second color at position ({x}|{y}) has a blue component of {blue:.2}%.",
        blue = color.blue,
        x = coordinate.0,
        y = coordinate.1
    );

    //
    // New hexbot with the parameters `count=3&seed=000000,0000FF`.
    //
    let hexbot_blue_only = fetch(3, Some(&[0x_00_00_00, 0x_00_00_FF]))?;
    println!("A hexbot with three hues of blue: {}", hexbot_blue_only);

    Ok(())
}
