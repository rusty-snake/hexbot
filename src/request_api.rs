/*
 * Copyright (C) 2019 rusty-snake <print_hello_world+License@protonmail.com>
 *
 * This file is part of my hexbot solution
 *
 * my hexbot solution is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * my hexbot solution is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Color {
    value: String,
}

#[derive(Debug, Deserialize)]
pub struct Hexbot {
    colors: [Color; 1],
}

impl Hexbot {
    pub fn get_color(&self) -> &str {
        &self.colors[0].value
    }
}

pub fn get_hexbot() -> Result<Hexbot, reqwest::Error> {
    Ok(reqwest::get("https://api.noopschallenge.com/hexbot")?.json::<Hexbot>()?)
}
