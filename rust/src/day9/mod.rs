
use crate::{get_file_path, helpers};
use std::{path::Path, sync::mpsc::{Sender, self, Receiver}, thread, collections::HashSet, ops::{Div}, time::{Duration, SystemTime}};
use image::*;
use blit::*;
use minifb::{WindowOptions, Scale, Window, Key};

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

    // println!("head moved to {}, {}", head.0, head.1);
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

enum GameSpeed {
    STEP, // not implemented -- only move head when triggered by user, or in ByKnot mode, only move a single knot
    SLOW, // 4 head moves per second (hmps)
    FAST, // 12 hmps
    BLAZE // 48 hmps
}

#[derive(Clone, Copy, PartialEq)]
enum RopeAnimationMode {
    WholeRope,
    ByKnot
}

#[derive(Clone)]
struct Knot {
    position: (i32, i32)
}

struct TailTracker {
    new_pos: (i32, i32),
    end_of_stream: bool
}

struct RopeGame {
    window: Window,
    animation_mode: RopeAnimationMode
}

#[derive(Clone)]
struct GameState {
    knots: Vec<Knot>
}

impl RopeGame {

    pub fn init_game() -> RopeGame {
        let window = RopeGame::setup_window(1280, 800);

        RopeGame { window, animation_mode: RopeAnimationMode::WholeRope }
    }

    fn setup_window(width: usize, height: usize) -> Window {
    
        let options = WindowOptions {
            scale: Scale::X1,
            ..WindowOptions::default()
        };
        let mut window = Window::new(
            "AOC Rope Bridge Animation - ESC to exit",
            width,
            height,
            options,
        )
        .expect("Unable to open window");
    
        window
    }

    fn get_knot_coords(knot_size: u8, window_size: (u32, u32), knot_position: (i32, i32)) -> (u32, u32) {
        // convert the X,Y of the knot to window coordinates based on knot size, window size and wrapping logic
        let mut x = knot_size as i32 * knot_position.0 + (window_size.0 / 2) as i32;

        // now implement wrapping
        while x < 0 {
            x += window_size.0 as i32;
        }

        while x > window_size.0 as i32 {
            x -= window_size.0 as i32;
        }

        let mut y = -1 * knot_size as i32 * knot_position.1 + (window_size.1 / 2) as i32;

        // wrapping again
        while y < 0 {
            y += window_size.1 as i32;
        }

        while y > window_size.1 as i32 {
            y -= window_size.1 as i32;
        }
        (x as u32 ,y as u32)
    }

    pub fn run_game(&mut self) -> Result<String, ImageError>{

        // and the messaging, make a thread to track the state of the game, the main thread will animate
        // the animation channel will receive a message when a knot moves (ByKnot Animation Mode) or before each head move (WholeRope)
        let (state_tx, state_rx) = mpsc::channel::<GameState>();

        // and we still need our tail tracking thread
        let (tail_tx, tail_rx) = mpsc::channel::<TailTracker>();
        let main_thread_tail_tx = tail_tx.clone();
        let tracker_handle = thread::spawn(move || {
            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            visited.insert((0,0));
            let mut track = tail_rx.recv().unwrap();
            while track.end_of_stream == false {
                // println!("tail moved to {}, {}", track.new_pos.0, track.new_pos.1);
                visited.insert(track.new_pos);
                track = tail_rx.recv().unwrap();
            }
            return visited.len();
        });

        let animation_mode = self.animation_mode;
        let speed = GameSpeed::BLAZE;

        // start the state handling thread
        let state_handle = thread::spawn(move || {

            // set up knots
            let knots = vec![Knot { position: (0,0)}; 10];
            let state = & mut GameState { knots };


            let steps = get_steps();
            for step in steps {
                // this is our main loop for driving the head, which drives the rest of the knots
                let start_time = SystemTime::now();

                // move the head
                let mut head = state.knots.get_mut(0).unwrap();
                match step {
                    DIRECTION::RIGHT => head.position.0 += 1,
                    DIRECTION::LEFT => head.position.0 -= 1,
                    DIRECTION::UP => head.position.1 += 1,
                    DIRECTION::DOWN => head.position.1 -= 1,
                }
                

                // now the rest of the rope
                for i in 1..state.knots.len() { // already did 0
                    if animation_mode == RopeAnimationMode::ByKnot {
                        let result = state_tx.send(state.clone());
                        if result.is_err() {
                            panic!("Could not animate a step!");
                        }
                    }

                    
                    state.knots[i].position = move_knot(state.knots[i-1].position, state.knots[i].position, &tail_tx);
                }

                let result = state_tx.send(state.clone());
                if result.is_err() {
                    panic!("Could not animate a step!");
                }

                let mut max_loop_time = match speed {
                    GameSpeed::SLOW => 25,
                    GameSpeed::FAST => 5,
                    GameSpeed::BLAZE => 1,
                    _ => 25
                };
                if animation_mode == RopeAnimationMode::WholeRope {
                    max_loop_time = max_loop_time * 4;
                }
                // SLOW is 4 or 40 fps
                let sleep_time = SystemTime::now().duration_since(start_time).unwrap();
                thread::sleep(Duration::from_millis(max_loop_time).checked_sub(sleep_time).unwrap_or(Duration::ZERO));
            }

            // close the tracker thread
            let sent = tail_tx.send(TailTracker { new_pos: (0,0), end_of_stream: true });

            if sent.is_err() {
                panic!("Failed to send!");
            }
        });

        let window = & mut self.window;

        
        let (width, height) = window.get_size();

        const MASK_COLOR: u32 = 0xFF_FF_FF;

        let mut buffer: Vec<u32> = vec![0x00_FF_FF_FF; width * height];
        let white_square_buffer = BlitBuffer::from_buffer(&[0x00_FF_FF_FF; 120*120], 120, 0x00_00_00_00);

        let img = image::open(get_file_path!("/resources/head.png")).unwrap();
        let img = img.resize(12, 12, imageops::FilterType::Nearest);
        println!("Loaded RGB image with size {:?}", img.dimensions());
        let img_size = img.dimensions();
    
        let rgb = img.into_rgb8().to_blit_buffer(MASK_COLOR);
    
        let x_pos = (width.div(2) as u32 - img_size.0 / 2) as i32;
        let y_pos = (height.div(2) as u32 - img_size.1 / 2) as i32;
    
        rgb.blit(&mut buffer, width, (x_pos, y_pos));

        let mut old_state: Option<GameState> = None;
        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&buffer, window.get_size().0, window.get_size().1).unwrap();
            
    
            // see if there any updates to positions and process
            if let Ok(new_state) = state_rx.try_recv() {
                // if recv then clear buffer and redraw with state in buffer
                // calculate where the white square should be applied
                let up_left: (i32, i32) = match old_state {
                    Some(old_state) => {
                        // since it's indexed from the top left corner we need to find the smallest x but the largest y
                        let min_knot = old_state.knots.iter().fold((i32::MAX, i32::MIN), |acc, item| {
                            let mut next = acc;
                            if item.position.0 < acc.0 {
                                next.0 = item.position.0;
                            }
                            if item.position.1 > acc.1 {
                                next.1 = item.position.1;
                            }
                            next
                        });
                        min_knot
                    },
                    _ => (0,0)
                };
                white_square_buffer.blit(&mut buffer, width, (up_left.0 * 12 + x_pos, -1 * (up_left.1 * 12) + y_pos));
                
                for knot in new_state.knots.as_slice() {
                    rgb.blit(&mut buffer, width, (knot.position.0 * 12 + x_pos, -1 * (knot.position.1 * 12) + y_pos));
                }
                old_state = Some(new_state.clone());
                thread::sleep(Duration::from_millis(20));
            }


        }

        

        if !tracker_handle.is_finished() {
            // kill the other threads to exit immediately
            main_thread_tail_tx.send(TailTracker { new_pos: (0, 0), end_of_stream: true }).expect("Could not kill tail thread");
            thread::sleep(Duration::from_millis(20));
        }
        let visited = tracker_handle.join().expect("TailTracker thread did not exit gracefully.");
        println!("Visited a total of {} spaces", visited);
        drop(state_rx);
        if !state_handle.is_finished() {
            // give a little more time for the state handler thread to exit after we drop the channel
            thread::sleep(Duration::from_millis(500));
        }
        match state_handle.join().is_ok() {
            true => {
                println!("State Thread finished.");
            },
            false => {
                println!("State Thread was killed.");
            }
        }

        Ok("EXIT 0".to_string())
    }

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
            // println!("tail moved to {}, {}", track.new_pos.0, track.new_pos.1);
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
    // now let's have some real fun!

    let mut game = RopeGame::init_game();
    let exit_msg = game.run_game();
    match exit_msg {
        Ok(exit_msg) => println!("{}", exit_msg),
        Err(exit_msg) => panic!("{:?}", exit_msg)
    }

}