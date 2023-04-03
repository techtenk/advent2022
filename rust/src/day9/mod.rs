
use crate::{get_file_path, helpers};
use std::{path::Path, sync::mpsc::{Sender, self}, thread, collections::HashSet};


#[derive(Clone,Copy)]
enum DIRECTION {
    RIGHT,
    LEFT,
    UP,
    DOWN
}

fn get_steps() -> Vec<DIRECTION> {
    let input_file = get_file_path!("input.txt");
    let mut vec_buffer = Vec::new();
    let lines = helpers::get_input_lines(&input_file.to_string(), & mut vec_buffer);

    let mut steps = Vec::new();
    // pase the input into discrete steps for the head
    for line in lines {
        if let Ok(line) = line {
            let line_split = line.split_whitespace().collect::<Vec<&str>>();
            let direction = line_split.first().unwrap().chars().nth(0).unwrap();
            let count = (*line_split.last().unwrap()).parse::<i32>().unwrap();
            let step = match direction {
                'R' => DIRECTION::RIGHT,
                'L' => DIRECTION::LEFT,
                'U' => DIRECTION::UP,
                'D' => DIRECTION::DOWN,
                _ => panic!("Encountered an unknown direction!")
            };
            for _ in 0..count {
                steps.push(step);
            }
            
        }
    }

    steps
}

fn move_knot(head: (i32, i32), tail: (i32, i32), tx: &Sender<TailTracker>) -> (i32, i32) {

    println!("head moved to {}, {}", head.0, head.1);
    if tail.0.abs_diff(head.0) <= 1 && tail.1.abs_diff(head.1) <= 1 {
        // println!("Tail remains at {}, {}", tail.0, tail.1);
        return tail;
    }

    let mut new_tail = tail;
    // two rules if head is not touching, otherwise do nothing
    // 1: if head is in same row, column, move tail one toward it
    // 2: if head is not in same row or column, move 1 in each direction towards it (diagonally)
    if tail.0 > head.0 {
        new_tail.0 -= 1;
    } else if tail.0 < head.0 {
        new_tail.0 += 1;
    }

    if tail.1 > head.1 {
        new_tail.1 -= 1;
    } else if tail.1 < head.1 {
        new_tail.1 += 1;
    }

    // println!("moving tail to {}, {}", new_tail.0, new_tail.1);
    let sent = tx.send(TailTracker {
        new_pos: new_tail,
        end_of_stream: false
    });
    if sent.is_err() {
        panic!("Failed to send message");
    }

    new_tail
}

struct TailTracker {
    new_pos: (i32, i32),
    end_of_stream: bool
}

pub fn run_part1() {
    let steps = get_steps();

    let mut head_pos = (0,0);
    let mut tail_pos = (0,0);

    // try out a thread messaging
    let (tx, rx) = mpsc::channel::<TailTracker>();

    // spawn a thread for listening to tail moves and tracking 
    let handle = thread::spawn(move || {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((0,0));
        let mut track = rx.recv().unwrap();
        while track.end_of_stream == false {
            println!("tail moved to {}, {}", track.new_pos.0, track.new_pos.1);
            visited.insert(track.new_pos);
            track = rx.recv().unwrap();
        }
        return visited.len();
    });

    for step in steps {
        match step {
            DIRECTION::RIGHT => head_pos.0 += 1,
            DIRECTION::LEFT => head_pos.0 -= 1,
            DIRECTION::UP => head_pos.1 += 1,
            DIRECTION::DOWN => head_pos.1 -= 1,
        }
        tail_pos = move_knot(head_pos, tail_pos, &tx);
    }

    
    let sent = tx.send(TailTracker { new_pos: (0,0), end_of_stream: true });

    if sent.is_err() {
        panic!("Failed to send!");
    }

    let visited = handle.join().unwrap();
    println!("tail visited {} spaces!", visited);
}

pub fn run_part2() {

}