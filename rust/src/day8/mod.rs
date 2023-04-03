use crate::{helpers, get_file_path};
use std::path::Path;

fn run() -> [[u8; 99]; 99] {
    let full_input_path = get_file_path!("input.txt");
    let mut vec_buf: Vec<u8> = Vec::new();
    let lines = helpers::get_input_lines(&full_input_path, &mut vec_buf);
    let mut trees: [[u8; 99]; 99] = [[0; 99]; 99];
    for (i, line) in lines.enumerate() {
        if let Ok(line) = line {
            for (j, c) in line.chars().enumerate() {
                trees[i][j] = c.to_string().parse::<u8>().unwrap();
            }
        }
    }

    trees

}

pub fn run_part1() {
    let trees = run();
    // now find which trees are visible
    let mut num_visible = 0;
    for i in 0..trees[0].len() {
        for j in 0..trees[0].len() {
            // easiest rule first, if it's on the outside count it visible
            if i == 0 || j == 0 || i == trees[0].len() - 1 || j == trees[0].len() - 1 {
                num_visible += 1;
                continue;
            }

            // otherwise we check all the neighbors very inefficiently
            // check top
            let mut vis_left = true;
            for k in 0..i {
                if trees[k][j] >= trees[i][j] {
                    vis_left = false;
                    break;
                }
            }

            // check bottom
            let mut vis_right = true;
            for k in i+1..trees[0].len() {
                if trees[k][j] >= trees[i][j] {
                    vis_right = false;
                    break;
                }
            }

            // check left
            let mut vis_north = true;
            for l in 0..j {
                if trees[i][l] >= trees[i][j] {
                    vis_north = false;
                    break;
                }
            }

            // check right
            let mut vis_south = true;
            for l in j+1..trees[0].len() {
                if trees[i][l] >= trees[i][j] {
                    vis_south = false;
                    break;
                }
            }

            if vis_left || vis_right || vis_north || vis_south {
                num_visible += 1;
            }
            
        }
    }

    println!("Number of trees visible: {}", num_visible);
}

pub fn run_part2() {
    let trees = run();

    // calculate scenic score for each tree, find the max
    let mut max_scenic_score = 0;
    // start at 1 and stop 1 early because edges will have a score of 0
    for i in 1..trees[0].len() - 1{
        for j in 1..trees[0].len() - 1 {
            // in this variation we have to start from the tree itself and look outward so we need to count down sometimes
            let current_tree = trees[i][j]; // for readability
            let mut current_tree_score = 0;
            let left_tree_score;
            let right_tree_score;
            let top_tree_score;
            let bottom_tree_score;

            let mut debug = false;
            if i == 3 && j == 2 {
                debug = true;
            }

            if debug { println!("Looking at tree with height: {}", current_tree)}
            // look top
            for k in (0..=i-1).rev() {
                if debug { println!("inspecting ({}, {}) height {}", k, j, trees[k][j])}
                current_tree_score += 1;
                if trees[k][j] >= current_tree {
                    if debug { println!("view blocked!")}
                    break;
                }
            }
            left_tree_score = current_tree_score;
            current_tree_score = 0;

            // look bottom
            for k in i+1..trees.len() {
                if debug { println!("inspecting ({}, {}) height {}", k, j, trees[k][j])}
                current_tree_score += 1;
                if trees[k][j] >= current_tree {
                    if debug { println!("view blocked!")}
                    break;
                }
            }

            right_tree_score = current_tree_score;
            current_tree_score = 0;

            // look left
            for l in (0..=j-1).rev() {
                if debug { println!("inspecting ({}, {}) height {}", i, l, trees[i][l])}
                current_tree_score += 1;
                if trees[i][l] >= current_tree {
                    if debug { println!("view blocked!")}
                    break;
                }
            }
            top_tree_score = current_tree_score;
            current_tree_score = 0;

            // look right
            for l in j+1..trees[0].len() {
                if debug { println!("inspecting ({}, {}) height {}", i, l, trees[i][l])}
                current_tree_score += 1;
                if trees[i][l] >= current_tree {
                    if debug { println!("view blocked!")}
                    break;
                }
            }
            
            bottom_tree_score = current_tree_score;
            current_tree_score = left_tree_score * right_tree_score * top_tree_score * bottom_tree_score; 

            if current_tree_score > max_scenic_score {
                max_scenic_score = current_tree_score;
            }
        }
    }

    println!("Max scenic score: {}", max_scenic_score);
}