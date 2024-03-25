extern crate image;
extern crate minifb;

use image::{ImageBuffer, Rgb};
use minifb::{Window, WindowOptions};

pub struct Viewer {
    window: Window,
}

impl Viewer {
    pub fn new(width: usize, height: usize) -> Self {
        

        Self {
            window: view_window,
        }
    }

    pub fn update(&im_buffer: ImageBuffer) {
        let pix_buffer: Vec<u32> = im_buffer.as_raw();

        self.window.update_with_buffer(pix_buffer);
    }
}
