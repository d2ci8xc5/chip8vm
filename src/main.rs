mod cpu;
mod keyboard;
mod graphics;

use cpu::cpu as _cpu;

fn main() {
  // setup graphics and input
  let CPU = _cpu::new();
  // init chip

  // load game

  loop {
    // emulate cycle

    // draw if draw flag is set

    // store key press state
  }

}
