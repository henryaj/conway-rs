extern crate pxl;

use pxl::*;

struct Conway {
    frame_num: u32,
}

impl Conway {
    fn get_color(&self) -> (f32, f32, f32) {
        let frame = self.frame_num as f32;
        let frame_step = 1.0 / 60.0;
        let red = frame * frame_step;
        return (red, 0.0, 0.0);
    }

    fn tick_frame(&mut self) {
        if self.frame_num > 60 {
            self.frame_num = 1;
        }

        self.frame_num += 1;
    }
}

impl Program for Conway {
    fn new() -> Conway {
        Conway { 
            frame_num: 0
        }
    }

    fn dimensions() -> (usize, usize) {
        return (700, 500);
    }

    fn render(&mut self, _pixels: &mut [Pixel]) {
        for pixel in _pixels.iter_mut() {
            let colors = self.get_color();

            pixel.red = colors.0;
            pixel.blue = colors.1;
            pixel.green = colors.2;
        }
        self.tick_frame();
        self.frame_num += 1;
    }
}

fn main() {
    pxl::run::<Conway>();
}