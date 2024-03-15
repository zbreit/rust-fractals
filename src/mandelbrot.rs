use crate::nums::Complex;

#[derive(Debug)]
pub enum CalcResult {
    Bounded,
    BailedOut(u16),
}

// Computes the mandelbrot sequence for an input number c
pub fn mandelbrot(c: Complex, max_iterations: u16, escape_magnitude: f64) -> CalcResult {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..max_iterations {
        z = z * z + c;

        // println!("{}, {:?}, {:?}, {}", i, c, z, z.mag());

        if z.mag() > escape_magnitude {
            return CalcResult::BailedOut(i);
        }
    }

    CalcResult::Bounded
}
