use crate::helpers;
use std::{fs, path::Path};

fn run() {
    let full_input_path = Path::new(file!()).parent().unwrap().as_os_str().to_str().unwrap().to_string() + "/" + "sample.txt";
    let mut array_buf: [u8; 4096] = [0; 4096];
    println!("{:?}", helpers::get_input_lines(&full_input_path, &mut array_buf).next().unwrap());
}

pub fn run_part1() {
    run();
}

pub fn run_part2() {

}