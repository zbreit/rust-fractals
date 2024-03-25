use num_traits::Float;

pub fn clamp<T: PartialOrd>(num: T, min: T, max: T) -> T {
    if num < min {
        return min;
    }

    if num > max {
        return max;
    }

    num
}

/// Scales x expotentially (basically computing b^c(x - 1)).
/// x is in the range 0-1 and the output is in the range 0-1.
pub fn exp_scaler(x: f32, b: f32, c: f32) -> f32 {
    let y_intercept = b.powf(-c);

    (b.powf(c * (x - 1.0)) - y_intercept) / (1.0 - y_intercept)
}

/// Converts x from the range x0-x1 into the range y0-y1. x0 is not necessary
/// less than x1
pub fn map_range<T: Float>(x: T, x0: T, x1: T, y0: T, y1: T) -> T {
    (x - x0) / (x1 - x0) * (y1 - y0) + y0
}
