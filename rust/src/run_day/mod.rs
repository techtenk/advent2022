// pub const DAY_PART_1: u8 = 1;
// pub const DAY_PART_2: u8 = 2;
pub const DAY_ALL_PARTS: u8 = 3;
use crate::day5;
use crate::day6;
use crate::day7;
use crate::day8;
use crate::day9;
use crate::day10;
use crate::day11;
use crate::day12;

pub fn run(day: i32, parts: u8) {
    let run_part1 = parts & 1 > 0;
    let run_part2 = parts & 2 > 0;

    match day {
        5 => {
            if run_part1 {
                day5::run_part1();
            }
            if run_part2 {
                day5::run_part2();
            }
        },
        6 => {
            if run_part1 {
                day6::run_part1();
            }
            if run_part2 {
                day6::run_part2();
            }
        },
        7 => {
            if run_part1 {
                day7::run_part1();
            }
            if run_part2 {
                day7::run_part2();
            }
        },
        8 => {
            if run_part1 {
                day8::run_part1();
            }
            if run_part2 {
                day8::run_part2();
            }
        },
        9 => {
            if run_part1 {
                day9::run_part1();
            }
            if run_part2 {
                day9::run_part2();
            }
        },
        10 => {
            if run_part1 {
                day10::run_part1();
            }
            if run_part2 {
                day10::run_part2();
            }
        },
        11 => {
            if run_part1 {
                day11::run_part1();
            }
            if run_part2 {
                day11::run_part2();
            }
        },
        12 => {
            if run_part1 {
                day12::run_part1();
            }
            if run_part2 {
                day12::run_part2();
            }
        },
        _ => {
            println!("Day {} not implemented", day);
        }
    }
}