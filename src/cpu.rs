use crate::font::font;
use crate::keypad::keypad;
use crate::display::display;

use sdl2::Sdl;
use rand::Rng;

pub struct cpu {
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    i: usize,  //u16
    pc: usize, //u16
    pub gfx: [u8; 64 * 32],
    pub delay_timer: u8,
    pub sound_timer: u8,
    stack: [usize; 16], //u16
    sp: usize,          //u16
    key: [bool; 16],

    redraw_flag: bool,
    keypad_wait_flag: bool,
    keypad_register: u8,
    keypad: keypad,
    display: display
}

impl cpu {
    pub fn new(sdl_context: &Sdl) -> cpu {
        let mut cpu = cpu {
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [false; 16],

            redraw_flag: false,
            keypad_wait_flag: false,
            keypad_register: 0, 
            keypad: keypad::new(sdl_context),
            display: display::new(sdl_context),
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

        // slice hex to four nibbles 
        // (a,b,c,d) = opcode
        let a: u8 = ((opcode & 0xf000) >> 12) as u8;
        let b: u8 = ((opcode & 0x0f00) >> 8) as u8;
        let c: u8 = ((opcode & 0x00f0) >> 4) as u8;
        let d: u8 = ((opcode & 0x000f) >> 0) as u8;

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
        match (a, b, c, d) {
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            (0x01, _, _, _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x03, _, _, _) => self.op_3xkk(x, kk),
            (0x04, _, _, _) => self.op_4xkk(x, kk),
            (0x05, _, _, 0x00) => self.op_5xy0(x, y),
            (0x06, _, _, _) => self.op_6xkk(x, kk),
            (0x07, _, _, _) => self.op_7xkk(x, kk),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8xy6(x, y),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.op_8xyE(x, y),
            (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            (0x0a, _, _, _) => self.op_Annn(nnn),
            (0x0b, _, _, _) => self.op_Bnnn(nnn),
            (0x0c, _, _, _) => self.op_Cxkk(x, kk),
            (0x0d, _, _, _) => self.op_Dxyn(x, y, n as u16),
            (0x0e, _, 0x09, 0x0e) => self.op_Ex9E(x),
            (0x0e, _, 0x0a, 0x01) => self.op_ExA1(x),
            (0x0f, _, 0x0a, 0x07) => self.op_Fx07(x),
            (0x0f, _, 0x00, 0x0a) => self.op_Fx0A(x),
            (0x0f, _, 0x01, 0x05) => self.op_Fx15(x),
            (0x0f, _, 0x01, 0x08) => self.op_Fx18(x),
            (0x0f, _, 0x01, 0x0e) => self.op_Fx1E(x),
            (0x0f, _, 0x02, 0x09) => self.op_Fx29(x),
            (0x0f, _, 0x03, 0x03) => self.op_Fx33(x),
            (0x0f, _, 0x03, 0x03) => self.op_Fx33(x),
            (0x0f, _, 0x05, 0x05) => self.op_Fx55(x),
            (0x0f, _, 0x06, 0x05) => self.op_Fx65(x),
            _ => println!("unknown opcode: {}{}{}{}", a, b, c, d), // make sure to append to pc so that it doesn't run the same (unknown) opcode again
        }
        // check if display must be redrawn
        if self.redraw_flag {
            // redraw display
            //self.display.draw(&mut self);
            // reset draw flag
            self.redraw_flag = false;
        }

        // check if cpu required to fetch keypad status
        if self.keypad_wait_flag {
            let poll_keypad;
            match self.keypad.poll() {
                Ok(x) => poll_keypad = x,
                Err(x) => panic!("{:?}", x),
            }

            // set register to value
            //self.memory[self.keypad_register] = key;
            self.keypad_wait_flag = false;
        }
        // update timers
    }

    pub fn fetch_opcode(&self) -> u16 {
        (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16)
    }

    // CLS - clear the display
    fn op_00e0(&mut self) {
        for i in 0..64 * 32 {
            self.gfx[i] = 0;
        }
        self.pc += 2;
        //self.redraw_flag = true;
    }

    // RET - return from a subroutine
    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }

    // JP nnn - jump to location nnn
    fn op_1nnn(&mut self, nnn: usize) {
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
    fn op_4xkk(&mut self, x: usize, kk: u8) {
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
        self.v[0x0f] = (self.v[x] & (1 << 7)) >> 7;
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
        self.v[0x0f] = (self.v[x] & (1 << 7) >> 7);
        self.v[x] <<= 1;
        self.pc += 2;
    }

    // SNE Vx, Vy
    fn op_9xy0(&mut self, x: usize, y: usize) {
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
    fn op_Cxkk(&mut self, x: usize, kk: u8) {
        let random_byte = rand::thread_rng().gen::<u8>();
        self.v[x] = (kk & random_byte);
        self.pc += 2;
    }

    // DRW Vx, Vy, nibble
    fn op_Dxyn(&mut self, x: usize, y: usize, n: u16) {
        self.v[0x0f] = 0;
        for y_offset in 0..n {
            let pixel = self.memory[(self.i + (y_offset as usize))];

            for x_offset in 0..8 {
                if (pixel & (0x80 >> x_offset)) != 0 {
                    let cur_pos = ((x as u16 + x_offset as u16)
                        + ((y as u16 + y_offset as u16) * 64))
                        % (32 * 64);

                    if self.gfx[cur_pos as usize] == 1 {
                        self.v[0x0f] = 1;
                    }

                    self.gfx[cur_pos as usize] ^= 1;
                }
            }
        }

        // redraw scene
        self.redraw_flag = true;
        self.pc += 2;
    }

    // SKP Vx
    fn op_Ex9E(&mut self, x: usize) {
        if self.key[self.v[x] as usize] == true {
            // key is currently pressed
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // SKNP Vx
    fn op_ExA1(&mut self, x: usize) {
        if self.key[self.v[x] as usize] == false {
            // key is not currently pressed
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // LD Vx, DT
    fn op_Fx07(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
        self.pc += 2;
    }

    // wait for keypress implement keypad
    // LD Vx, DT
    fn op_Fx0A(&mut self, x: usize) {
        self.keypad_wait_flag = true;
        // cache the keypad register to write at the next cycle
        self.keypad_register = (x as u8);
        self.pc += 2;
    }

    // LD DT, Vx
    fn op_Fx15(&mut self, x: usize) {
        self.delay_timer = self.v[x];
        self.pc += 2;
    }

    // LD ST, Vx
    fn op_Fx18(&mut self, x: usize) {
        self.sound_timer = self.v[x];
        self.pc += 2;
    }

    // ADD I, Vx
    fn op_Fx1E(&mut self, x: usize) {
        self.i += self.v[x] as usize;
        self.pc += 2;
    }

    // LD F, Vx
    fn op_Fx29(&mut self, x: usize) {
        self.i = (self.v[x] as usize) * 5;
        self.pc += 2;
    }

    // LD B, Vx
    fn op_Fx33(&mut self, x: usize) {
        self.memory[self.i] = self.v[x] / 100;
        self.memory[self.i + 1] = (self.v[x] % 100) / 10;
        self.memory[self.i + 2] = self.v[x] % 10;
        self.pc += 2;
    }

    fn op_Fx55(&mut self, x: usize) {
        for i in 0..x + 1 {
            self.memory[self.i + i] = self.v[i];
        }
        self.pc += 2;
    }

    fn op_Fx65(&mut self, x: usize) {
        for i in 0..x + 1 {
            self.v[i] = self.memory[self.i + i];
        }
        self.pc += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdl2; 
    #[test]
    fn test_new() {
        let sdl_context = sdl2::init().unwrap();      
        let cpuvm = cpu::new(&sdl_context);
        assert_eq!(cpuvm.opcode, 0u16);
        for (i, byte) in font.iter().enumerate() {
            assert_eq!(*byte, font[i]);
        }
        //assert_eq!(cpuvm.v, );
        assert_eq!(cpuvm.i, 0usize);
        //assert_eq!(cpuvm.pc, 0usize);
        //assert_eq!(cpuvm.gfx, [0u8; u8]);
        //assert_eq!(cpuvm.delay_timer, 0u8);
        //assert_eq!(cpuvm.sound_timer, 0u8);
        //assert_eq!(cpuvm.stack, [0, 16]);
        //assert_eq!(cpuvm.sp, 0usize);
        //assert_eq!(cpuvm.key, [0u8; 16]);
    }

//    #[test]
//    fn test_cycle() {
//        let sdl_context = sdl2::init().unwrap();      
//        let cpuvm = cpu::new(&sdl_context);
//    }
//
//    #[test]
//    fn test_fetch_opcode() {
//        let sdl_context = sdl2::init().unwrap();      
//        let cpuvm = cpu::new(&sdl_context);
//    }
}
