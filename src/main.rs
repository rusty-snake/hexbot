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

mod request_api;

fn main() {
    let hexbot = request_api::get_hexbot().unwrap();
    println!("Hexbot responded with color {}.", hexbot);

    //dbg!(request_api::get_hexbot().unwrap().get_color().to_rgb255());
    //dbg!(request_api::get_hexbot().unwrap().get_color().red);
}
