use sdl2::pixels::Color;

pub enum color {
    red,
    green,
    blue,
    black,
}

impl color {
    pub fn value(&self) -> Color {
        match *self {
            color::red => Color::RGB(255, 0, 0),
            color::green => Color::RGB(0, 255, 0),
            color::blue => Color::RGB(0, 0, 255),
            color::black => Color::RGB(0, 0, 0),
        }
    }
}
