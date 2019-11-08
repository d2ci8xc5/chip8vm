mod color;
mod cpu;
mod display;
mod font;
mod keypad;

use cpu::cpu as _cpu;
use std::env;
use sdl2;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().len() == 1 {
    }
    for arg in args.iter() {
        println!("{}", arg);
    }
    // check for debug
    // setup graphics and input
    let sdl_context = sdl2::init().unwrap();
    // init chip
    //let mut cpuvm = _cpu::new(&sdl_context);
    // load game
    loop {
        // emulate cycle
        //cpu_vm.cycle();

        // draw if draw flag is set

        // store key press state
    }
}
