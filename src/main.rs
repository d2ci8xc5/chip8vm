mod cpu;
mod keypad;
mod display;
mod font;

use sdl2;
use cpu::cpu as _cpu;

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
