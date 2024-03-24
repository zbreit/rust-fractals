use crate::nums::Complex;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum CalcResult {
    Bounded,
    BailedOut(u16),
}

// Computes the mandelbrot sequence for an input number c
pub fn mandelbrot(c: Complex, max_iterations: u16, escape_magnitude: f64) -> CalcResult {
    let mut z = Complex { re: 0.0, im: 0.0 };

    let escape_sqr_mag = escape_magnitude.powi(2);

    for i in 0..max_iterations {
        z = z * z + c;

        // Here, we use square magnitude to avoid calculating a square root every
        // calculation
        if z.square_mag() > escape_sqr_mag {
            return CalcResult::BailedOut(i);
        }
    }

    CalcResult::Bounded
}
