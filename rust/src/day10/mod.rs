use std::io::{BufReader, Lines};

use crate::{get_file_path, helpers};

pub fn run_part1() {
    let mut buf = Vec::new();
    let lines = helpers::get_input_lines(get_file_path!("sample.txt"), & mut buf);

    let iss: Vec<i32> = Vec::new();
    let mut cycle = 1;
    let mut register = 0;
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

            
        }
    }
}

pub fn run_part2() {

}