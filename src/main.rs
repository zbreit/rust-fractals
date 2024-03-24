use image::{Rgb, RgbImage};
use mandelbrot::CalcResult;
use math::{clamp, exp_scaler, map_range};
use palette::{Hsv, IntoColor, Srgb};
use std::{collections::HashSet, ops::Sub};

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

fn rgb_from_index(index: u16, max_iters: u16) -> Rgb<u8> {
    let div: f64 = (std::u8::MAX as f64) * 3.0 * (index as f64);

    let sub: f64 = div / (max_iters as f64);

    let red: u8 = clamp(sub, 0.0, 255.0) as u8;
    let green: u8 = clamp(sub - 255.0, 0.0, 255.0) as u8;
    let blue: u8 = clamp(sub - 510.0, 0.0, 255.0) as u8;

    Rgb::<u8>([red, green, blue])
}

fn rainbow_from_index(index: u16, max_iters: u16) -> Rgb<u8> {
    const SATURATION: f32 = 0.65;
    const START_VALUE: f32 = 0.75;
    const MAX_VALUE: f32 = 0.85;
    const START_HUE: f32 = 0.0;
    const MAX_HUE: f32 = 60.0;

    let raw_percent = (index as f32) / (max_iters as f32);
    let percent = exp_scaler(raw_percent, 2.0, 1.0);

    let hue = map_range(percent, 0.0, 1.0, START_HUE, MAX_HUE);
    let value = map_range(percent, 0.0, 1.0, START_VALUE, MAX_VALUE);

    let hsv: Hsv = Hsv::new_srgb(hue, SATURATION, value);
    let rgb: Srgb = hsv.into_color();
    let rgb_u8: Srgb<u8> = rgb.into_format();

    Rgb::from([rgb_u8.red, rgb_u8.green, rgb_u8.blue])
}

fn color_from_result(result: CalcResult, max_iters: u16) -> Rgb<u8> {
    match result {
        CalcResult::Bounded => Rgb::<u8>([0, 0, 0]),
        CalcResult::BailedOut(i) => rainbow_from_index(i, max_iters),
    }
}

fn draw_mandelbrot(width: u32, height: u32, bounds: Rect<f64>) -> RgbImage {
    const MAX_ITERS: u16 = 50;
    const ESCAPE_MAG: f64 = 1e6;
    let mut image = RgbImage::new(width, height);

    let mut unique_results = HashSet::new();

    for x in 0..width {
        for y in 0..height {
            let pct_x: f64 = (x as f64) / (width as f64);
            let pct_y: f64 = (y as f64) / (height as f64);
            let c = Complex {
                re: bounds.left + pct_x * bounds.width(),
                im: bounds.top - pct_y * bounds.height(),
            };
            let result = mandelbrot(c, MAX_ITERS, ESCAPE_MAG);

            unique_results.insert(result);

            // println!("{}, {}, {:?}, {:?}", x, y, c, result);

            let color = color_from_result(result, MAX_ITERS);

            image.put_pixel(x, y, color);
        }
    }

    let unique_bail_outs: Vec<u16> = unique_results
        .into_iter()
        .filter(|result| !matches!(result, CalcResult::Bounded))
        .map(|result| match result {
            CalcResult::Bounded => 0,
            CalcResult::BailedOut(i) => i,
        })
        .collect();

    let min: u16 = *unique_bail_outs.iter().min().unwrap();
    let max: u16 = *unique_bail_outs.iter().max().unwrap();

    println!("Num Unique Colors: {}", unique_bail_outs.len());
    println!("Color-Range: {} - {}", min, max);
    println!("Max Iters: {}", MAX_ITERS);
    println!("Escape: {}", ESCAPE_MAG);
    image
}

fn main() {
    let bounds = Rect::<f64> {
        top: -1.25,
        bottom: 1.25,
        left: -2.0,
        right: 0.5,
    };

    let image = draw_mandelbrot(2048, 2048, bounds);

    // TODO: handle error case if image fails to save
    image.save("./mandelbrot.png").unwrap();
}
