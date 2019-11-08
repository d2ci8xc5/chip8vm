mod color;
mod cpu;
mod display;
mod font;
mod keypad;

use cpu::cpu as _cpu;
extern crate sdl2;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 || args.len() > 1 {
        println!("ROM not parsed");
    }
    for arg in args.iter() {
        println!("{}", arg);
    }
    // check for debug
    // setup graphics and input
    let sdl_context = sdl2::init().unwrap();
    // init chip
    let mut cpuvm = _cpu::new(&sdl_context);
    // load game
    loop {
        // emulate cycle
        //cpu_vm.cycle();

        // draw if draw flag is set

        // store key press state
    }
}
