mod cpu;
mod display;
mod font;
mod keypad;

use cpu::cpu as _cpu;
use sdl2;

fn main() {
    // setup graphics and input
    let mut cpu_vm = _cpu::new();
    // init chip

    // load game

    loop {
        // emulate cycle
        cpu_vm.cycle();

        // draw if draw flag is set

        // store key press state
    }
}
