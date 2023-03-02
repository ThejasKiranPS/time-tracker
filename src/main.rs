use std::env;
use time_tracker::{log, Args, Config};

fn main() {
    let args = Args::build(env::args()).expect("Didn't get enough arguments!");

    let config = Config::default();

    log(args, config).expect("Couldn't log action!");
}
