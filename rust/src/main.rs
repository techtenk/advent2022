use std::env;

mod run_day;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod helpers;

fn help() {
    println!("usage:
aoc22 --help
    Print this help message
aoc22 run [day]
    day can be:
      - a positive integer to run a single day
      - 'all' to run all days
      - latest (default) to run just the day with the highest index value
Examples:
    aoc run 3
    aoc run latest
    aoc run all
      ");
}

#[cfg(feature = "image")]
fn main() {
    // check command line args, print usage if incorrect or has --help 
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            // a single parameter can be --help or run
            if !args[1].eq("run") {
                help();
            } else {
                println!("Run latest");
            }
        },
        3 => {
            if args[1].eq("run") {
                match args[2].as_str() {
                    "latest" => {
                        println!("Run latest");
                    },
                    "all"  => {
                        println!("Run all");
                    },
                    _ => {
                        // try to parse it as an integer, if it's not an integer, 0 won't match a day
                        let num = args[2].parse::<i32>().unwrap_or(0);
                        println!("Running day {}", num);
                        run_day::run(num, run_day::DAY_ALL_PARTS);
                    }
                }
            } else {
                help();
            }
        }
        _ => {
            help();
        }
    }
}
