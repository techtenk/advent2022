use std::{collections::HashMap, ops::{Sub, Add}};

use crate::{get_file_path, helpers};

/* more fun! implement a new trait for i32 */
pub trait RelevantCycle<T: Sub + Eq> {
    fn is_relevant(&self) -> bool;
}

impl RelevantCycle<usize> for usize {
    fn is_relevant(&self) -> bool {
        // if it's 20 or a multiple of 40 after then it's a cycle we care about, i.e. 20, 60, 100, etc...
        let mut remaining = *self + 20;

        while remaining >= 40 {
            remaining -= 40;
        }
        remaining == 0
    }
}

pub fn run_part1() {
    let mut buf = Vec::new();
    let lines = helpers::get_input_lines(&get_file_path!("input.txt"), & mut buf).collect::<Vec<Result<_, _>>>();

    let mut iss: Vec<i32> = Vec::new();
    let mut cycle = 1;
    let mut register = 1;

    let mut relevant_cycles: HashMap<usize, i32> = HashMap::new();

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
                relevant_cycles.insert(cycle, cycle as i32 * register);
            } else if (cycle + 1).is_relevant() {
                relevant_cycles.insert(cycle + 1, (cycle + 1) as i32 * register);
            }
        }
    }
    println!("Checkpoints: \n {:?}", relevant_cycles);
    let total = relevant_cycles.iter().fold(0, |acc, item| {
        acc + *item.1
    });
    println!("Total: {}", total);
}

pub fn run_part2() {
    
}