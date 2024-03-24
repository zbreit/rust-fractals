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

/// Scales x expotentially (basically computing b^c(x - 1)).
/// x is in the range 0-1 and the output is in the range 0-1.
pub fn map_range(x: f32, min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> f32 {
    (x - min_x) / (max_x - min_x) * (max_y - min_y) + min_y
}
