use std::{io::{BufReader, Lines, BufRead}, iter::Peekable};
// use regex::Regex;


fn get_input() -> Box<&'static [u8]> {
    Box::new(include_bytes!("input.txt"))
}

fn get_initial_stacks() -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    // start by parsing the input until we get through the initial stacks definition
    let mut no_moves = true;
    // let first_move = None;
    let mut lines: Peekable<Lines<BufReader<&[u8]>>> = BufReader::new(*get_input()).lines().peekable();

    while no_moves {
        
        if let Some(Ok(mut line)) = lines.next() {
            let mut drain_n = 3;
            let mut current_stack = 0;
            // let letters_regex = Regex::new("[A-Z]").unwrap();
            while line.len() > 0 {
                let elf_crate: String = line.drain(..drain_n).collect();
                if elf_crate.contains('[') {
                    while stacks.len() <= current_stack {
                        stacks.push(Vec::new());
                    }
                    stacks[current_stack].push(elf_crate.chars().reduce(|current, next| if next.is_alphabetic() { return next; } else { return current; }).unwrap());
                }
                current_stack += 1;
                drain_n = 4;
            }
        }
        if let Some(next_line) = lines.peek().clone() {
            match next_line {
                Ok(next_line) => {
                    no_moves = !next_line.contains("move");
                },
                _ => {
                    no_moves = false;
                }
            }
        } else {
            no_moves = false;
        }

    }
    stacks
}

#[derive(Clone,Copy)]
enum CraneModel {
    CRATEMOVER9000,
    CRATEMOVER9001
}

fn simulate_moves(stacks: &mut Vec<Vec<char>>, count: u32, from: u32, to: u32, crane: CraneModel) {
    let mut temp_vec: Vec<char> = Vec::new();
    {
    
        let mut block_to_move = stacks[(from-1) as usize].drain(0..count as usize);
        while let Some(blk) = block_to_move.next() {
            temp_vec.push(blk);
        }
    }
    // now that the mutable lifetime of stacks has run out, update the destination
    let blk_iterator: Box<dyn DoubleEndedIterator<Item = &char>> = match crane { CraneModel::CRATEMOVER9000 => { Box::new(temp_vec.iter()) },  CraneModel::CRATEMOVER9001 => { Box::new(temp_vec.iter().rev())}};
    for blk in blk_iterator {
        stacks[(to-1) as usize].insert(0, *blk);
    }
}

pub fn prepare_cmds() -> Vec<(u32, u32, u32)> {
    let mut cmd_list = Vec::new();
    for line in BufReader::new(get_input()).lines() {
        match line {
            Ok(l) => {
                if l.starts_with("move") {
                    // filter to just the numbers, the last digit is the destination, etc..
                    let nums_string = l.chars().filter(|c| char::is_numeric(*c)).collect::<String>();
                    let nums: &str = &nums_string[..].as_ref();
                    let (nums, t) = nums.split_at(nums.len() - 1);
                    let (nums, f) = nums.split_at(nums.len() - 1);
                    
                    cmd_list.push((nums.parse::<u32>().unwrap(), f.parse::<u32>().unwrap(), t.parse::<u32>().unwrap()));
                }
            },
            _ => {
                continue;
            }
        }
    }
    cmd_list
}

fn run(crane: &CraneModel) {
    let mut stacks = get_initial_stacks();

    // println!("Got my stacks");
    // for stack in &stacks {
    //     println!("{:?}", stack);
    // }

    // simulate each command
    for cmd in prepare_cmds() {
        let (count, f, t) = cmd;
        simulate_moves(&mut stacks, count, f, t, *crane);
        // println!("After {}", l);
        // for stack in &stacks {
        //     println!("{:?}", stack);
        // }
    }


    println!("Read off the top letters: ");
    for stack in &stacks {
        print!("{}", stack[0]);
    }
    println!("");
}

pub fn run_part1() {
    run(&CraneModel::CRATEMOVER9000);
}

pub fn run_part2() {
    run(&CraneModel::CRATEMOVER9001);
}