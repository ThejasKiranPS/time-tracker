use chrono::{Duration, Local, NaiveDateTime};
use dirs::data_local_dir;
use rev_lines::RevLines;
use std::fs::create_dir;
use std::io::BufReader;
use std::path::PathBuf;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};
const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub struct Config {
    pub data_file_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let mut path = PathBuf::from(data_local_dir().unwrap());
        path.push("time-tracker");
        path.push("logs");
        Self {
            data_file_path: path,
        }
    }
}
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

fn open_file(config: Config) -> File {
    let path = Path::new(&config.data_file_path);

    // TODO: remove repetition
    if path.exists() {
        OpenOptions::new()
            .read(true)
            .append(true)
            .open(&path)
            .expect("Could not open file!")
    } else {
        create_dir(path.parent().unwrap()).expect("Couldn't create directory");
        OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&path)
            .expect("Could not open file!")
    }
}

/// Logs current time along with the `mode` to `file`
pub fn write(mut file: &File, mode: &str) {
    file.write_fmt(format_args!("{0} {1}\n", get_current_time(), mode))
        .expect("Could not write to file");
}

pub fn log(args: Args, config: Config) -> Result<(), &'static str> {
    if !["start", "end"].contains(&&*args.mode) {
        return Err("Invalid mode! Please provide one of the following: `start` or `end`");
    }

    let mode = args.mode;
    let file = open_file(config);

    write(&file, &mode);
    if mode == "end" {
        let duration = calculate_time_difference(&file).unwrap();
        println!("{}", format_duration(duration));
    }
    Ok(())
}

pub fn calculate_time_difference(file: &File) -> Option<chrono::Duration> {
    let mut rev_lines = RevLines::new(BufReader::new(file)).unwrap();
    let end_line = rev_lines.next()?;

    let (date_time, mode) = parse_line(end_line).unwrap();
    let end_time = parse_time(&date_time);
    if mode != "end" {
        panic!("End record wasn't logged properly");
    }

    for line in rev_lines {
        let (date_time, mode) = parse_line(line).unwrap();
        if mode == "start" {
            let start_time = parse_time(&date_time);
            return Some(end_time - start_time);
        }
    }
    None
}

fn parse_time(date_time: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(&date_time, TIME_FORMAT).expect("Could not parse time")
}

fn parse_line(line: String) -> Option<(String, String)> {
    let mut parsed_line = line.split(' ');
    let date = parsed_line.next()?;
    let time = parsed_line.next()?;
    let mode = parsed_line.next()?;

    Some((format!("{} {}", date, time), mode.to_string()))
}

/// Returns current date & time in a readable format
pub fn get_current_time() -> String {
    let now = Local::now();
    format!("{}", now.format(TIME_FORMAT))
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.num_seconds() % 60;
    let minutes = duration.num_minutes() % 60;
    let hours = duration.num_hours();
    format!("{}h {}m {}s", hours, minutes, seconds)
}
