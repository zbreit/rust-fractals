use image::{Rgb, RgbImage};
use mandelbrot::CalcResult;
use math::clamp;
use std::ops::Sub;

use crate::{mandelbrot::mandelbrot, nums::Complex};

mod mandelbrot;
mod math;
mod nums;

pub struct Rect<T: Sub> {
    top: T,
    left: T,
    bottom: T,
    right: T,
}

impl Rect<f64> {
    #[inline]
    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    #[inline]
    pub fn height(&self) -> f64 {
        self.top - self.bottom
    }
}

fn color_from_index(index: u16, max_iters: u16) -> Rgb<u8> {
    let div: f64 = (std::u8::MAX as f64) * 3.0 * (index as f64);

    let sub: f64 = div / (max_iters as f64);

    let red: u8 = clamp(sub, 0.0, 255.0) as u8;
    let green: u8 = clamp(sub - 256.0, 0.0, 255.0) as u8;
    let blue: u8 = clamp(sub - 512.0, 0.0, 255.0) as u8;

    Rgb::<u8>([red, green, blue])
}

fn color_from_result(result: CalcResult, max_iters: u16) -> Rgb<u8> {
    match result {
        CalcResult::Bounded => Rgb::<u8>([0, 0, 0]),
        CalcResult::BailedOut(i) => color_from_index(i, max_iters),
    }
}

fn draw_mandelbrot(width: u32, height: u32, bounds: Rect<f64>) -> RgbImage {
    let mut image = RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let pct_x: f64 = (x as f64) / (width as f64);
            let pct_y: f64 = (y as f64) / (height as f64);
            let c = Complex {
                re: bounds.left + pct_x * bounds.width(),
                im: bounds.top - pct_y * bounds.height(),
            };

            let max_iters = 100;
            let result = mandelbrot(c, max_iters, 1e4);

            // println!("{}, {}, {:?}, {:?}", x, y, c, result);

            let color = color_from_result(result, max_iters);

            image.put_pixel(x, y, color);
        }
    }

    image
}

fn main() {
    let bounds = Rect::<f64> {
        top: 1.5,
        left: -2.0,
        right: 1.0,
        bottom: -1.5,
    };

    let image = draw_mandelbrot(2048, 2048, bounds);

    // TODO: handle error case if image fails to save
    image.save("./mandelbrot.png").unwrap();
}
