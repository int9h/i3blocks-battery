/*
 *  Copyright (C) 2017  Manuel Wildauer <m.wildauer@gmail.com>
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;

fn main() {

    let battery = "BAT0";

    let color_bad = "#FF0000";
    let color_yellow = "#FFAE00";
    let color_ok = "#A8FF00";

    /*
     * Build bath to 'battery'
     */
    let base_path = PathBuf::from(format!("/sys/class/power_supply/{}", battery));

    let capacity = match get_value(&base_path, "capacity").parse::<u32>() {
        Ok(c) => c,
        Err(e) => panic!("capacity is no integer: {}", e),
    };

    let status = get_value(&base_path, "status");
    let status = match status.as_str() {
        "Discharging" => "DIS",
        _ => &status,
    };

    println!("{}% {}", capacity, status);
    println!("{}% {}", capacity, status);

    let color = match capacity {
        _ if capacity < 10 => color_bad,
        _ if capacity < 50 => color_yellow,
        _ => color_ok
    };

    println!("{}", color);

    /*
     * set to urgent when capacity < 5%
     */
    if capacity < 5 {
        process::exit(33);
    }
}

fn get_value(base_path: &PathBuf, value: &str) -> String {
    let mut path = PathBuf::from(base_path);
    path.push(value);
    get_information(&path).trim_end().to_string()
}

fn get_information(path: &PathBuf) -> String {
    let mut file = File::open(&path).unwrap_or_else(|_| panic!("couldn't open {}", path.display()));
    let mut information = String::new();
    file.read_to_string(&mut information).unwrap_or_else(|_| panic!("couldn't read {}", path.display()));
    information
}
