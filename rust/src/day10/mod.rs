use std::{collections::HashMap, ops::{Sub, Add}, num::Wrapping};

use crate::{get_file_path, helpers};

/* more fun! implement a new trait for i32 */
trait RelevantCycle<T: Add + Sub + Eq> {
    fn is_relevant(&self) -> bool;
}

impl RelevantCycle<Wrapping<i32>> for i32 {
    fn is_relevant(&self) -> bool {
        // if it's 20 or a multiple of 40 after then it's a cycle we care about, i.e. 20, 60, 100, etc...
        if (*self + 20).wrapping_div(40) == 0 {
            return true;
        }
        false
    }
}

pub fn run_part1() {
    let mut buf = Vec::new();
    let lines = helpers::get_input_lines(get_file_path!("sample.txt"), & mut buf).collect();

    let iss: Vec<i32> = Vec::new();
    let mut cycle = 1;
    let mut register = 0;

    let relevant_cycles: HashMap<i32, Option<i32>> = HashMap::new();
    for i in (20..=lines.len()).step_by(40) {
        relevant_cycles.insert(i, None);
    }

    for line in lines {
        if let Ok(line) = line {
            let elements = line.split_whitespace().collect::<Vec<&str>>();
            let cmd = *elements.get(0).unwrap();
            

            match cmd {
                "addx" => {
                    let num = elements.get(1).unwrap().parse::<i32>().unwrap();
                    register += num;
                    cycle += 2;
                },
                "noop" => {
                    cycle += 1;
                },
                _ => {
                    panic!("unrecognized cmd '{}'", cmd);
                }
            }

            if cycle.is_relevant() {

            }
        }
    }
}

pub fn run_part2() {

}