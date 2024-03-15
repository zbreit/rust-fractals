use image::{ImageBuffer, Rgb, RgbImage};
use mandelbrot::CalcResult;
use math::clamp;

use crate::{mandelbrot::mandelbrot, nums::Complex};

mod mandelbrot;
mod math;
mod nums;

pub struct Rect<T> {
    top: T,
    left: T,
    width: T,
    height: T,
}

fn color_from_index(index: u16, max_iters: u16) -> Rgb<u8> {
    let div: f64 = (std::u8::MAX as f64) * 3.0;

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
                re: pct_x / bounds.width - bounds.left,
                im: pct_y / bounds.height - bounds.top,
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
        top: 1.12,
        left: -2.0,
        width: 2.0,
        height: 2.0,
    };

    let image = draw_mandelbrot(512, 512, bounds);

    // TODO: handle error case
    image.save("./mandelbrot.png").unwrap();
}
