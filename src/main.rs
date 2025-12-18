extern crate core;

mod days;
mod util;

use std::env::args;
use std::time::Instant;
use days::{get_day, Day};
use util::input::{read_input};
use crate::util::number::parse_usize;

fn print_usage()
{
    eprintln!("
Usage: cargo run <command> [<command_arg>, ...]

Commands:
    day <day number> - run the puzzles for the given day.
    add <day number> - add base files and wiring for a new day.
    race             - race through implemented days, keeping track of time.
");
}

fn main() {
    let a: Vec<String> = args().collect();

    match a[1].as_str() {
        "race" if a.len() == 2 => {
            let mut day = 1;
            let start = Instant::now();

            while run_day(day) {
                day += 1;
            }

            println!("Finished AoC race: {}ms", Instant::now().duration_since(start).as_millis());
        }
        "day" if a.len() == 3 => {
            let Some(day) = parse_usize(&a[2]).ok() else { panic!("Invalid day number {}", &a[2]) };
            run_day(day);
        }
        "add" if a.len() == 3 => {
            let Some(day) = parse_usize(&a[2]).ok() else { panic!("Invalid day number {}", &a[2]) };
            add_day(day);
        }
        _ => {
            print_usage();
        }
    }
}

fn run_day(day_num: usize) -> bool
{
    let result: Result<(String, Day), String> = get_day(day_num).and_then(|day| read_input(day_num).and_then(|input| Ok((input, day))));
    match result {
        Ok((input, day)) => {
            let p1_start = Instant::now();
            match (day.puzzle1)(&input) {
                Ok(res) => {
                    println!("Day {} part 1 result: {} (took {}ms)", day_num, res, Instant::now().duration_since(p1_start).as_millis());
                },
                Err(err) => {
                    eprintln!("Day {} part 1 failed: {} (took {}ms)", day_num, err, Instant::now().duration_since(p1_start).as_millis());
                }
            }

            let p2_start = Instant::now();
            match (day.puzzle2)(&input) {
                Ok(res) => {
                    println!("Day {} part 2 result: {} (took {}ms)", day_num, res, Instant::now().duration_since(p2_start).as_millis());
                },
                Err(err) => {
                    eprintln!("Day {} part 2 failed: {} (took {}ms)", day_num, err, Instant::now().duration_since(p1_start).as_millis());
                }
            }

            true
        }
        Err(err) => {
            eprintln!("{}", err);
            false
        }
    }
}

fn add_day(day: usize)
{
    // This is going to be fun. Write code to modify the running code! Woohoo!
    match util::create_day::create_day(day) {
        Ok(_) => { println!("Successfully added day {}", day); }
        Err(e) => { panic!("{}", e); }
    }
}