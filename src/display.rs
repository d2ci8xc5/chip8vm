use crate::color::color;
use crate::cpu::cpu;

use sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
const SCALE: u32 = 25;
pub struct display {
    canvas: Canvas<Window>,
}

impl display {
    pub fn new(sdl_context: &sdl2::Sdl) -> display {
        let video_subsystem = sdl_context.video().unwrap();
        let window: Window = video_subsystem
            .window("chip8vm", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(color::black.value());
        canvas.clear();
        canvas.present();

        display { canvas: canvas }
    }

    // draw sprite
    pub fn draw(&mut self, gfx: [u8; 32*64]) {
        for y in 0..32 {
            for x in 0..64 {
                let x_true = (x as u32) * SCALE; 
                let y_true = (y as u32) * SCALE; 

               
                let mut draw_color = color::black.value();
                if gfx[y*64 + x] == 1 {

                    draw_color = color::green.value();
                } else {
                    draw_color = color::black.value();
                }
                self.canvas.set_draw_color(draw_color);
                let rect_draw = self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE, SCALE));

                match rect_draw {
                    Ok(x) => continue,
                    Err(x) => panic!("{:?}", x),
                }
            }
        }
    }

    pub fn clear(&mut self, gfx: [u8; 32*64]) {
         
    
    }
}
