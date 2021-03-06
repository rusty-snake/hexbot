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

use hexbot::{Count, Hexbot, Seed, WidthHeight};
use std::io;
use std::io::Write;

fn input(prompt: &str) -> io::Result<String> {
    let mut buffer = String::new();
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    stdout.write_all(prompt.as_bytes())?;
    stdout.flush()?;
    stdin.read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn ask_bool(question: &str) -> io::Result<bool> {
    let answer = input(question)?.to_lowercase();
    if answer == "y" || answer == "yes" {
        Ok(true)
    } else if answer == "n" || answer == "no" {
        Ok(false)
    } else {
        println!("Sorry, try again.");
        ask_bool(question)
    }
}

fn ask_i32(question: &str) -> io::Result<i32> {
    if let Ok(answer) = input(question)?.parse() {
        Ok(answer)
    } else {
        println!("Sorry, try again.");
        ask_i32(question)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===== Hexbot =====");
    let count = if ask_bool("Should the count parameter be added? [yes|no] ")? {
        loop {
            match Count::yes(ask_i32("What value should count have? [1-1000] ")?) {
                Ok(count) => break count,
                Err(_) => println!("This is a invalid count, try again."),
            }
        }
    } else {
        Count::no()
    };
    let with_coordinates =
        if ask_bool("Should the width and height parameters be added? [yes|no] ")? {
            loop {
                match WidthHeight::yes(
                    ask_i32("What value should width have? [10-100,000] ")?,
                    ask_i32("What value should height have? [10-100,000] ")?,
                ) {
                    Ok(limit) => break limit,
                    Err(_) => println!("This is a invalid width/height, try again."),
                }
            }
        } else {
            WidthHeight::no()
        };
    let hb = Hexbot::fetch(count, with_coordinates, &Seed::no()).await.expect("Fetching failed");
    println!("{}", hb);
    Ok(())
}
