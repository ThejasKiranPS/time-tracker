use chrono::{format::Item, Local};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};
pub struct Config {
    pub path: String,
}

impl Config {}
pub struct Args {
    pub mode: String,
}

impl Args {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        // skipping filename
        args.next();

        let mode = match args.next() {
            Some(arg) => arg.to_lowercase(),
            None => return Err("Didn't get mode (start/end)."),
        };

        Ok(Args { mode })
    }
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

/// Logs current time along with the `mode` to `file`
pub fn write(mut file: File, mode: String) {
    file.write_fmt(format_args!("{0} {1}\n", get_current_time(), mode))
        .expect("Could not write to file");
}

pub fn log(args: Args) -> Result<(), &'static str> {
    if !["start", "end"].contains(&&*args.mode) {
        return Err("Invalid mode! Please provide one of the following: `start` or `end`");
    }

    let mode = args.mode;
    let file = open_file("hello.txt");

    write(file, mode);
    Ok(())
}

pub fn calculate_time_difference() {}

/// Returns current date & time in a readable format
pub fn get_current_time() -> String {
    let now = Local::now();
    format!("{}", now.format("%Y-%m-%d %H:%M:%S"))
}
