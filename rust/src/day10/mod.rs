use std::{collections::HashMap, ops::{Sub}, time::{Duration, SystemTime}, thread};

use blit::{BlitBuffer, BlitExt};
use image::{Rgb, RgbImage};
use minifb::{WindowOptions, Scale, Window, Key};
use rusttype::{Font, Scale as RustTypeScale};

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

    // let mut iss: Vec<i32> = Vec::new();
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

fn setup_window(width: usize, height: usize) -> Window {
    
    let options = WindowOptions {
        scale: Scale::X1,
        ..WindowOptions::default()
    };
    let window = Window::new(
        "AoC 2022 CRT Display - ESC to exit",
        width,
        height,
        options,
    )
    .expect("Unable to open window");

    window
}

pub fn run_part2() {
    // how fun! a drawing one right after I set up a drawing system in the last one

    // "pixel are 20x20 
    let window = setup_window(800, 240);

    // set up Image for the canvas

    // set up blit buffer for printing header cursor "XXX"
    // set up blit buffer for printed spaces - a black square
    let mask = 0x00_FF_FF_FF;

    // black square, 20x20
    let black_square: &[u32; 400] = &[0x00_00_00_00; 400];

    // blit buffer for drawing the "pixels"
    let black_blitter: BlitBuffer = BlitBuffer::from_buffer(black_square, 20, mask);

    // position of the sprite will be represented with three gray '#'
    let font = Vec::from(include_bytes!("../resources/whiterabbit/whitrabt.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();
    let text_scale = RustTypeScale {
        x: 20.0 * 2.0,
        y: 20.0,
    };

    let printhead_imgbuf = imageproc::drawing::draw_text(&RgbImage::new(60, 20), Rgb::from([0, 0, 0]), 0, 0, text_scale, &font, "###");
    let printhead_blitter: BlitBuffer = printhead_imgbuf.to_blit_buffer(mask);
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start_time = SystemTime::now();

        // receive update events, if any
        // if try_recv
        // clear old sprites, draw new sprites

        let max_loop_time = 25; // max 40 fps
        let sleep_time = SystemTime::now().duration_since(start_time).unwrap();
        thread::sleep(Duration::from_millis(max_loop_time).checked_sub(sleep_time).unwrap_or(Duration::ZERO));
    }

}
