use crate::font::font;
use crate::keypad;
use crate::display;

pub struct cpu {
  opcode: u16,
  memory : [u8; 4096],
  v: [u8; 16],
  i: u16,
  pc: u16,
  gfx: [u8; 64*32],
  delay_timer: u8,
  sound_timer: u8,
  stack: [u16; 16],
  sp: u16,
  key: [u8; 16]
  //keypad: keypad, 
  //display: display 
}

impl cpu {
  pub fn new() -> cpu {
    let mut cpu = cpu {
      opcode: 0,
      memory: [0; 4096],
      v: [0; 16],
      i: 0,
      pc: 0x200,
      gfx: [0; 64*32],
      delay_timer: 0,
      sound_timer: 0,
      stack: [0; 16],
      sp: 0,
      key: [0; 16]
      //keypad: keypad::new(),
      //display: display::new() 
    };

    // init memory to font
    for i in 0..80 {
      cpu.memory[i] = font[i];
    }
    cpu
  }

  fn cycle(&mut self) {
    // fetch opcode
    
    //let opcode: u16 =  
    // decode opcode

    // execute opcode

    // update timers

  }

  fn op_00e0(&mut self) {
    for i in 0..64*32 {
      self.gfx[i] = 0;
    }
  }

  fn op_00ee(&mut self) {
    self.sp -= 1;
    //self.pc = self.stack[self.sp];
  }
  
  fn op_1nnn(&mut self) {
    //self.pc = nnn; 
  }
}
