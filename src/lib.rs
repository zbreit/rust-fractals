use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    re: f64,
    im: f64,
}

pub impl Complex {
    pub fn conjugate(self) -> Self {}
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Div for Complex {
    type Output = Self;

    // Divide by another number is just multiplying by the complex conjucate
    fn div(self, rhs: Self) -> Self::Output {}
}

#[cfg(test)]
mod tests {
    use crate::Complex;

    const DELTA: f64 = std::f64::EPSILON * 10.0;

    #[test]
    fn add() {
        let a = Complex { re: 2.5, im: 4.3 };
        let b = Complex { re: 20.0, im: 10.0 };
        let result = a + b;
        let expect = Complex { re: 20.5, im: 14.3 };

        assert!((result.re - expect.re).abs() < DELTA);
        assert!((result.im - expect.im).abs() < DELTA);
    }
}
