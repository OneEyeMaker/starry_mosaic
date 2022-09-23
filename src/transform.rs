use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use super::utility;

#[derive(Clone, Copy, Debug)]
pub struct Scale {
    pub x: f64,
    pub y: f64,
}

impl Scale {
    #[inline(always)]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn new_uniform(scale: f64) -> Self {
        Self { x: scale, y: scale }
    }

    pub fn clamp(&self, minimum_scale: f64, maximum_scale: f64) -> Self {
        assert!(minimum_scale >= 0.0);
        Self {
            x: self.x.signum() * self.x.abs().clamp(minimum_scale, maximum_scale),
            y: self.y.signum() * self.y.abs().clamp(minimum_scale, maximum_scale),
        }
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self { x: 1.0, y: 1.0 }
    }
}

impl From<f64> for Scale {
    fn from(scale: f64) -> Self {
        Self { x: scale, y: scale }
    }
}
impl From<(f64, f64)> for Scale {
    fn from(scale: (f64, f64)) -> Self {
        Self {
            x: scale.0,
            y: scale.1,
        }
    }
}

impl PartialEq for Scale {
    fn eq(&self, scale: &Self) -> bool {
        utility::approx_eq(self.x, scale.x) && utility::approx_eq(self.y, scale.y)
    }
}

impl Add for Scale {
    type Output = Scale;
    fn add(self, scale: Self) -> Self::Output {
        Scale {
            x: self.x * scale.x,
            y: self.y * scale.y,
        }
    }
}
impl Sub for Scale {
    type Output = Scale;
    fn sub(self, scale: Self) -> Self::Output {
        Scale {
            x: self.x / scale.x,
            y: self.y / scale.y,
        }
    }
}

impl Neg for Scale {
    type Output = Scale;
    fn neg(self) -> Self::Output {
        Scale {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl AddAssign for Scale {
    fn add_assign(&mut self, scale: Self) {
        self.x *= scale.x;
        self.y *= scale.y;
    }
}
impl SubAssign for Scale {
    fn sub_assign(&mut self, scale: Self) {
        self.x /= scale.x;
        self.y /= scale.y;
    }
}
