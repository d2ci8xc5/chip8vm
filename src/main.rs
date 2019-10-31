mod cpu;
mod keypad;
mod display;
mod font;

use sdl2;
use cpu::cpu as _cpu;

fn main() {
  // setup graphics and input
  let sdl_context = sdl2::init().unwrap();
  
  let cpuvm = _cpu::new();
  // init chip

  // load game

  loop {
    // emulate cycle

    // draw if draw flag is set

    // store key press state
  }

}
