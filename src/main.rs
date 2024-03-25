use bounds::Rect;
use image::{Rgb, RgbImage};
use mandelbrot::CalcResult;
use math::{clamp, exp_scaler, map_range};
use minifb::{Key, MouseMode, Window, WindowOptions};
use palette::{Hsv, IntoColor, Srgb};
use std::{collections::HashSet, ops::Sub};
use std::{thread, time};

use crate::{mandelbrot::mandelbrot, nums::Complex};

mod bounds;
mod mandelbrot;
mod math;
mod nums;

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
        CalcResult::BailedOut(i) => rgb_from_index(i, max_iters),
    }
}

fn draw_mandelbrot(width: u32, bounds: &Rect<f64>) -> RgbImage {
    const MAX_ITERS: u16 = 200;
    const ESCAPE_MAG: f64 = 1e6;
    let height = ((width as f64) / bounds.aspect_ratio()) as u32;
    let mut image = RgbImage::new(width, height);

    // let mut unique_results = HashSet::new();

    for x in 0..width {
        for y in 0..height {
            let pct_x: f64 = (x as f64) / (width as f64);
            let pct_y: f64 = (y as f64) / (height as f64);
            let c = Complex {
                re: bounds.left + pct_x * bounds.width(),
                im: bounds.top - pct_y * bounds.height(),
            };
            let result = mandelbrot(c, MAX_ITERS, ESCAPE_MAG);

            // unique_results.insert(result);

            let color = color_from_result(result, MAX_ITERS);

            image.put_pixel(x, y, color);
        }
    }

    // let unique_bail_outs: Vec<u16> = unique_results
    //     .into_iter()
    //     .filter(|result| !matches!(result, CalcResult::Bounded))
    //     .map(|result| match result {
    //         CalcResult::Bounded => 0,
    //         CalcResult::BailedOut(i) => i,
    //     })
    //     .collect();

    // let min: u16 = *unique_bail_outs.iter().min().unwrap();
    // let max: u16 = *unique_bail_outs.iter().max().unwrap();

    // println!("Num Unique Colors: {}", unique_bail_outs.len());
    // println!("Color-Range: {} - {}", min, max);
    // println!("Max Iters: {}", MAX_ITERS);
    // println!("Escape: {}", ESCAPE_MAG);
    image
}

fn gui_init(width: usize, height: usize) -> Window {
    let window: Window = Window::new("Hi Zach :)", width, height, WindowOptions::default())
        .expect("Unable to open Window");

    window
}

fn gui_update(window: &mut Window, im_buffer: &RgbImage) {
    let width: usize = usize::try_from(im_buffer.width()).unwrap();
    let height: usize = usize::try_from(im_buffer.height()).unwrap();

    let mut pix_buffer: Vec<u32> = Vec::with_capacity(width * height);

    for y in 0..im_buffer.height() {
        for x in 0..im_buffer.width() {
            let [red, green, blue] = im_buffer.get_pixel(x, y).0;

            let display_rgb: u32 = (red as u32) << 16 | (green as u32) << 8 | (blue as u32);
            pix_buffer.push(display_rgb);
        }
    }

    window
        .update_with_buffer(&pix_buffer, width, height)
        .unwrap();
}

fn main() {
    const ORIGINAL_BOUNDS: Rect<f64> = Rect::<f64> {
        top: 1.25,
        bottom: -1.25,
        left: -2.0,
        right: 0.5,
    };

    let mut bounds = ORIGINAL_BOUNDS;

    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;
    const ZOOM_SCALE_FACTOR: f64 = 0.1;

    let mut window: Window = gui_init(WIDTH, HEIGHT);
    let mut image = draw_mandelbrot(WIDTH as u32, &bounds);

    gui_update(&mut window, &image);

    let mut prev = time::Instant::now();
    let mut last_update = prev;

    let mut scale: f64 = 1.0;

    while window.is_open() {
        let now = time::Instant::now();
        let elapsed = now - prev;

        if (now - last_update).as_secs() >= 1 {
            // println!("FPS: {}", 1.0 / elapsed.as_secs_f32());

            last_update = now;
        }

        if let Some((_, scroll)) = window.get_scroll_wheel() {
            let (mouse_x, mouse_y) = window.get_mouse_pos(MouseMode::Clamp).unwrap_or_default();

            let zoom = ((scroll.abs() as f64) * ZOOM_SCALE_FACTOR).powf(scroll.signum() as f64);
            let x = map_range(mouse_x as f64, 0.0, WIDTH as f64, bounds.left, bounds.right);
            let y = map_range(
                mouse_y as f64,
                HEIGHT as f64,
                0.0,
                bounds.bottom,
                bounds.top,
            );

            // let (center_x, center_y) = bounds.midpoint();

            // let new_x = center_x - x / (zoom) + x;
            // let new_y = center_y - y / (zoom) + y;

            // scale *= zoom;

            // let old_bounds = bounds.clone();
            // bounds = bounds.scale(zoom).center_on(new_x, new_y);
            bounds = bounds.zoom_to((x, y), zoom);

            println!("scrolling - wx {mouse_x} wy {mouse_y} scroll {scroll}");
            println!("x {x} y {y} zoom {zoom}");
            // println!("nx {new_x} ny {new_y}");
            // println!("cx {center_x} cy {center_y}");
            // println!("old bounds {old_bounds:?}");
            println!("bounds {bounds:?}");
            println!();

            image = draw_mandelbrot(WIDTH as u32, &bounds);
            gui_update(&mut window, &image);

            continue;
        }

        if window
            .get_keys_pressed(minifb::KeyRepeat::No)
            .contains(&Key::R)
        {
            bounds = ORIGINAL_BOUNDS;
            image = draw_mandelbrot(WIDTH as u32, &bounds);
            gui_update(&mut window, &image);

            continue;
        }

        prev = now;

        window.update();
    }

    // TODO: handle error case if image fails to save
    image.save("./mandelbrot.png").unwrap();
}
