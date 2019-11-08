mod color;
mod cpu;
mod display;
mod font;
mod keypad;

use cpu::cpu as _cpu;
use sdl2;
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
    //let mut cpu_vm = _cpu::new(sdl);
    // init chip

    // load game
    loop {
        // emulate cycle
        //cpu_vm.cycle();

        // draw if draw flag is set

        // store key press state
    }
}
