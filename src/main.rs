use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::path::Path;
use std::process;

fn main() {

    let battery = "BAT0";

    let color_bad = "#FF0000";
    let color_yellow = "#FFAE00";
    let color_ok = "#A8FF00";

    /**
     * Build bath to 'battery'
     */
    let path = Path::new("/sys/class/power_supply");
    let base_path = path.join(battery);

    let mut capacity = get_value(base_path.clone(), "capacity".to_string()).to_string();
    let len_capacity = capacity.len();
    capacity.truncate(len_capacity - 1);

    let mut status = get_value(base_path.clone(), "status".to_string()).to_string();
    let len_status = status.len();
    status.truncate(len_status - 1);

    if status == String::from("Discharging") {
        status = "DIS".to_string();
    }

    println!("{}% {}", capacity, status);
    println!("{}% {}", capacity, status);

    let mut color = String::new();
    if capacity < String::from("20") {
        color.push_str(color_bad);
    } else if capacity < String::from("50") {
        color.push_str(color_yellow);
    } else {
        color.push_str(color_ok);
    }

    println!("{}", color);

    /*
     * set to urgent when capacity < 5%
     */
    if capacity < String::from("5") {
        process::exit(33);
    }
}

fn get_value(base_path: PathBuf, value: String) -> String {
    let mut path = PathBuf::from(base_path);
    path.push(value);
    return get_information(path);
}

fn get_information(path: PathBuf) -> String {
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    let mut information = String::new();
    match file.read_to_string(&mut information) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why.description()),
        Ok(_) => return information,
    }
}
