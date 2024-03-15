use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn conjugate(self) -> Self {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn mag(self) -> f64 {
        self.re.powi(2) + self.im.powi(2)
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl Add<f64> for Complex {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re + rhs,
            im: self.im,
        }
    }
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

impl Sub<f64> for Complex {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re - rhs,
            im: self.im,
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

impl Mul<f64> for Complex {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
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

impl Div<f64> for Complex {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl Div for Complex {
    type Output = Self;

    // Divide by another number is just multiplying by the complex conjugate
    fn div(self, rhs: Self) -> Self::Output {
        let conj = rhs.conjugate();
        let norm = rhs.mag();

        self * conj / norm
    }
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
        let expect = Complex { re: 22.5, im: 14.3 };

        assert!((result.re - expect.re).abs() < DELTA);
        assert!((result.im - expect.im).abs() < DELTA);
    }
}
