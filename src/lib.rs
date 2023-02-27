use chrono::Local;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};
pub struct Config {
    pub path: String,
}
pub struct Args {
    pub mode: String,
}
fn open_file(path: &str) -> File {
    let path = Path::new(&path);
    if path.exists() {
        OpenOptions::new()
            .append(true)
            .open(&path)
            .expect("Could not open file!")
    } else {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .expect("Could not open file!")
    }
}
pub fn log(mut file: File, mode: String) {
    file.write_fmt(format_args!("{0} {1}\n", get_current_time(), mode))
        .expect("Could not write to file");
}

pub fn log_start() {
    let f = open_file("hello.txt");
    log(f, String::from("Start"));
}

pub fn log_end() {
    let f = open_file("hello.txt");
    log(f, String::from("End"));
}

pub fn calculate_time_difference() {}
pub fn get_current_time() -> String {
    let now = Local::now();
    format!("{}", now.format("%Y-%m-%d %H:%M:%S"))
}
