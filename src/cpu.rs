use crate::font::font;
use crate::keypad;
use crate::display;
use rand::Rng;

pub struct cpu {
  opcode: u16,
  memory : [u8; 4096],
  v: [u8; 16],
  i: usize, //u16
  pc: usize, //u16
  gfx: [u8; 64*32],
  delay_timer: u8,
  sound_timer: u8,
  stack: [usize; 16], //u16
  sp: usize, //u16
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

  pub fn cycle(&mut self) {
    // fetch opcode
    let opcode = self.fetch_opcode(); 
    
    // decode opcode
    //nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
    //n or nibble - A 4-bit value, the lowest 4 bits of the instruction
    //x - A 4-bit value, the lower 4 bits of the high byte of the instruction
    //y - A 4-bit value, the upper 4 bits of the low byte of the instruction
    //kk or byte - An 8-bit value, the lowest 8 bits of the instruction
    let nnn = (opcode & 0x0fff) as usize; 
    let n = (opcode & 0x000f) as usize;
    let x = (opcode & 0x0f00) as usize;
    let y = (opcode & 0x00f0) as usize;
    let kk = (opcode & 0x00ff) as u8;
    // execute opcode

    // update timers

  }

  pub fn fetch_opcode(&self) -> u16 {
    (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc+1] as u16)
  }

  // CLS - clear the display
  fn op_00e0(&mut self) {
    for i in 0..64*32 {
      self.gfx[i] = 0;
    }
    self.pc += 2;
  }

  // RET - return from a subroutine
  fn op_00ee(&mut self) {
    self.sp -= 1;
    self.pc = self.stack[self.sp];
  }
  
  // JP nnn - jump to location nnn
  fn op_1nnn(&mut self, nnn : usize) {
    self.pc = nnn;
  }
  
  // CALL nnn - call subroutine at nnn
  fn op_2nnn(&mut self, nnn: usize) {
    self.stack[self.sp] = self.pc;
    self.sp += 1;
    self.pc = nnn;
  }

  // SE Vx, byte - 3xkk - skip the next instruction if Vx = kk
  fn op_3xkk(&mut self, x: usize, kk: u8) {
    if self.v[x] == kk {
      self.pc += 4;
    } else {
      self.pc += 2; 
    }
  }

  // SE Vx, byte - 4xkk - skip the next instruction if Vx != kk
  fn op_4xkk(&mut self, x: usize, kk : u8) {
    if self.v[x] != kk {
      self.pc += 4;
    } else {
      self.pc += 2; 
    }
  }

  // SE Vx, Vy - 5xy0 - skip next instruction if Vx = Vy
  fn op_5xy0(&mut self, x: usize, y: usize) {
    if self.v[x] == self.v[y] {
      self.pc += 4;
    } else {
      self.pc += 2; 
    }
  }

  // LD Vx, byte - 6xkk - put kk into vx 
  fn op_6xkk(&mut self, x: usize, kk: u8) {
    self.v[x] = kk;
    self.pc += 2;
  }

  // ADD Vx, byte - 7xkk - add kk to Vx then stores in Vx
  fn op_7xkk(&mut self, x: usize, kk: u8) {
    self.v[x] += kk;
    self.pc += 2;
  }

  // LD Vx Vy - 8xy0 - stores value of register Vy into Vx 
  fn op_8xy0(&mut self, x: usize, y: usize) {
    self.v[x] = self.v[y];
    self.pc += 2;
  }

  // OR Vx, Vy - bitwise OR on Vx and Vy then store result in Vx
  fn op_8xy1(&mut self, x: usize, y: usize) {
    self.v[x] |= self.v[y];
    self.pc += 2;
  }

  // AND Vx, Vy - bitwise AND on Vx and Vy then store result in Vx
  fn op_8xy2(&mut self, x: usize, y: usize) {
    self.v[x] &= self.v[y];
    self.pc += 2;
  }

  // XOR Vx, Vy - bitwise XOR on Vx and Vy then store result in Vx
  fn op_8xy3(&mut self, x: usize, y: usize) {
    self.v[x] ^= self.v[y];
    self.pc += 2;
  }

  // ADD Vx, Vy  
  fn op_8xy4(&mut self, x: usize, y: usize) {
    let v_x = self.v[x] as u16;
    let v_y = self.v[y] as u16;
    let addition: u8 = (v_x + v_y) as u8;
    self.v[x] = addition;
    if addition > 0xff {
      self.v[0x0f] = 1;
    } else {
      self.v[0x0f] = 0;
    }
    self.pc += 2;
  }

  // SUB Vx, Vy
  fn op_8xy5(&mut self, x: usize, y: usize) {
    if self.v[x] > self.v[y] {
      self.v[0x0f] = 1;
    } else {
      self.v[0x0f] = 0;
    }
    self.v[x] = self.v[x].wrapping_sub(self.v[y]);
    self.pc += 2;
  }

  // SHR Vx, Vy
  fn op_8xy6(&mut self, x: usize, y: usize) {
    // MSB value check
    self.v[0x0f] =  (self.v[x] & ( 1 << 7)) >> 7;
    self.v[x] <<= 1;
    self.pc += 2;
  }

  // SUBN Vx, Vy
  fn op_8xy7(&mut self, x: usize, y: usize) {
    if self.v[y] > self.v[x] {
      self.v[0x0f] = 1;
    } else {
      self.v[0x0f] = 0;
    }
    self.v[x] = self.v[y].wrapping_sub(self.v[x]);
    self.pc += 2;
  }

  // SHL Vx, {, Vy} 
  fn op_8xyE(&mut self, x: usize, y: usize) {
    self.v[0x0f] = (self.v[x] & ( 1 << 7) >> 7);
    self.v[x] <<= 1;
    self.pc += 2;
  }

  // SNE Vx, Vy 
  fn op_9xyE(&mut self, x: usize, y: usize) {
    if self.v[x] != self.v[y] {
      // skip next instruction
      self.pc += 4;
    } else {
      self.pc += 2;
    }
  }

  // LD I, addr 
  fn op_Annn(&mut self, nnn: usize) {
    self.i = nnn;
    self.pc += 2;
  }

  // JP V0, addr 
  fn op_Bnnn(&mut self, nnn: usize) {
    self.pc = nnn + (self.v[0] as usize);
  }
  
  // RND Vx, byte
  fn op_Cxkk(&mut self, x: usize, kk : u8) {
    let random_byte = rand::thread_rng().gen::<u8>();
    self.v[x] = (kk & random_byte);   
    self.pc += 2;
  }

  // DRW Vx, Vy, nibble
  fn op_Dxyn(&mut self, x: usize, y: usize, n: u16) {
    self.v[0x0f] = 0;
  }

  // tests
}
