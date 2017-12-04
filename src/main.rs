use std::env;
use std::process;

mod day_1;
mod day_2;
mod day_3;
mod utils;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        error("Please pass in the day # to run.");
    }

    let day= &args[1];

    run_day(day);
}

fn error(message: &str) {
    println!("{0}", message);
    process::exit(1);
}

fn run_day(day: &str) {
    match day {
        "1" => day_1::run(),
        _ => error("Please pass in the day # to run."),
    }
}