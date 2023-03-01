use std::env;
use time_tracker::{log, Args};

fn main() {
    let args = Args::build(env::args()).expect("Didn't get enough arguments!");

    log(args).expect("Couldn't log action!");
}
