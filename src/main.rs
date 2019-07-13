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

// needs pub to generate documentation.
pub mod request_api;

fn main() {
    let hexbot = request_api::fetch(5).unwrap();
    println!("A hexbot with five colors: {}", hexbot);

    let mut red_sum = 0;
    for color in hexbot.colors() {
        red_sum += color.to_rgb255().0 as i32;
    }
    println!("The sum of all red values: {}", red_sum);

    let hexbot_with_coordinates = request_api::fetch_with_coordinates(5, 100, 100).unwrap();
    println!(
        "A hexbot with five colors and coordiantes: {}",
        hexbot_with_coordinates
    );

    let coordinate = hexbot_with_coordinates.coordinates().unwrap()[1];
    let color = hexbot_with_coordinates.colors()[1];
    println!(
        "The second color at position ({x}|{y}) has a blue component of {blue:.2}%.",
        blue = color.blue,
        x = coordinate.0,
        y = coordinate.1
    );
}
