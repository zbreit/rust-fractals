#[derive(Debug, Clone)]
pub struct Rect<T> {
    pub top: T,
    pub left: T,
    pub bottom: T,
    pub right: T,
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

    #[inline]
    pub fn aspect_ratio(&self) -> f64 {
        (self.right - self.left) / (self.top - self.bottom)
    }

    pub fn center_on(&self, x: f64, y: f64) -> Self {
        Rect {
            top: y + self.height() / 2.0,
            bottom: y - self.height() / 2.0,
            right: x + self.width() / 2.0,
            left: x - self.width() / 2.0,
        }
    }

    pub fn translate(&self, dx: f64, dy: f64) -> Self {
        Rect {
            top: self.top + dy,
            bottom: self.bottom + dy,
            right: self.right + dx,
            left: self.left + dx,
        }
    }

    pub fn scale(&self, zoom: f64) -> Self {
        let new_width = self.width() / zoom;
        let x_diff = (self.width() - new_width) / 2.0;
        let new_height = self.height() / zoom;
        let y_diff = (self.height() - new_height) / 2.0;

        Rect {
            top: self.top - y_diff,
            bottom: self.bottom + y_diff,
            right: self.right - x_diff,
            left: self.left + x_diff,
        }
    }

    pub fn midpoint(&self) -> (f64, f64) {
        (
            (self.left + self.right) / 2.0,
            (self.top + self.bottom) / 2.0,
        )
    }

    /// Zooms to a given point by the specified zoom amount. Maintains the relative position
    /// of the supplied point in the viewport.
    ///
    /// The equations here were derived as follows (for a single dimension):
    /// |--------.--------------------------------|
    /// L     l  m               r                R
    ///
    /// L, R are the original left/right bounds
    /// l, r are the new left/right bounds
    /// m is the zoom point
    /// z is the zoom factor
    /// p is m's percent in the viewport (see below for the calculation)
    ///
    /// We know m, L, R, and z. We can compute p directly. So we are solving for
    /// l and r using the equations below.
    /// 1. p = (m - L) / (R - L)    <-- Percent in original viewport from [0, 1]
    /// 2. m = L + p (R - L)        <-- m is p% in the original viewport
    /// 3. m = l + p (r - l)        <-- m is also p% in the new viewport
    /// 4. z = (R - L) / (r - l)    <-- The zoom factor scales the input range to the output range
    ///
    /// You can solve the system of equations via substitution and you get the equations below.
    pub fn zoom_to(&self, point: (f64, f64), zoom: f64) -> Self {
        let left = self.left + (point.0 - self.left) * (1.0 - 1.0 / zoom);
        let right = (point.0 - left) / (point.0 - self.left) * (self.right - self.left) + left;
        let bottom = self.bottom + (point.1 - self.bottom) * (1.0 - 1.0 / zoom);
        let top = (point.1 - bottom) / (point.1 - self.bottom) * (self.top - self.bottom) + bottom;

        Rect {
            left,
            right,
            top,
            bottom,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bounds::Rect;

    const DELTA: f64 = 1e-10;

    #[test]
    fn zoom_in() {
        let r = Rect::<f64> {
            left: 1.0,
            right: 2.0,
            top: 4.0,
            bottom: 2.0,
        };

        let zoomed = r.scale(2.0);

        let expect = Rect::<f64> {
            left: 1.25,
            right: 1.75,
            top: 3.5,
            bottom: 2.5,
        };

        assert!((zoomed.left - expect.left).abs() < DELTA);
        assert!((zoomed.right - expect.right).abs() < DELTA);
        assert!((zoomed.top - expect.top).abs() < DELTA);
        assert!((zoomed.bottom - expect.bottom).abs() < DELTA);
    }

    #[test]
    fn zoom_out() {
        let r = Rect::<f64> {
            left: 1.0,
            right: 2.0,
            top: 4.0,
            bottom: 2.0,
        };

        let zoomed = r.scale(0.5);

        let expect = Rect::<f64> {
            left: 0.5,
            right: 2.5,
            top: 5.0,
            bottom: 1.0,
        };

        assert!((zoomed.left - expect.left).abs() < DELTA);
        assert!((zoomed.right - expect.right).abs() < DELTA);
        assert!((zoomed.top - expect.top).abs() < DELTA);
        assert!((zoomed.bottom - expect.bottom).abs() < DELTA);
    }

    #[test]
    fn center_on() {
        let r = Rect::<f64> {
            left: 0.0,
            right: 10.0,
            top: 8.0,
            bottom: 0.0,
        };

        let centered = r.center_on(0.0, 0.0);

        let expect = Rect::<f64> {
            left: -5.0,
            right: 5.0,
            top: 4.0,
            bottom: -4.0,
        };

        assert!((centered.left - expect.left).abs() < DELTA);
        assert!((centered.right - expect.right).abs() < DELTA);
        assert!((centered.top - expect.top).abs() < DELTA);
        assert!((centered.bottom - expect.bottom).abs() < DELTA);
    }

    #[test]
    fn translate() {
        let r = Rect::<f64> {
            left: 0.0,
            right: 1.0,
            top: 2.0,
            bottom: 1.0,
        };

        let translated = r.translate(1.0, -2.0);

        let expect = Rect::<f64> {
            left: 1.0,
            right: 2.0,
            top: 0.0,
            bottom: -1.0,
        };

        assert!((translated.left - expect.left).abs() < DELTA);
        assert!((translated.right - expect.right).abs() < DELTA);
        assert!((translated.top - expect.top).abs() < DELTA);
        assert!((translated.bottom - expect.bottom).abs() < DELTA);
    }
}
