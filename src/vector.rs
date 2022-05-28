use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::utility;

#[derive(Clone, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    #[inline(always)]
    pub fn new(x: f64, y: f64) -> Self {
        Vector {
            x,
            y,
        }
    }
    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }
    #[inline(always)]
    pub fn squared_distance_to(&self, vector: &Vector) -> f64 {
        (self - vector).squared_length()
    }
    #[inline(always)]
    pub fn distance_to(&self, vector: &Vector) -> f64 {
        (self - vector).length()
    }
    pub fn get_normalized(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }
    pub fn dot(&self, vector: &Self) -> f64 {
        self.x * vector.x + self.y * vector.y
    }
    pub fn cross(&self, vector: &Self) -> f64 {
        self.y * vector.x - self.x * vector.y
    }
}

impl Debug for Vector {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.write_str("(")?;
        Debug::fmt(&self.x, formatter)?;
        formatter.write_str(", ")?;
        Debug::fmt(&self.y, formatter)?;
        formatter.write_str(")")
    }
}
impl Display for Vector {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.write_str("(")?;
        Display::fmt(&self.x, formatter)?;
        formatter.write_str(", ")?;
        Display::fmt(&self.y, formatter)?;
        formatter.write_str(")")
    }
}

impl From<(f64, f64)> for Vector {
    fn from(tuple: (f64, f64)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, vector: &Self) -> bool {
        utility::approx_eq(self.x, vector.x) && utility::approx_eq(self.y, vector.y)
    }
}
impl PartialOrd for Vector {
    fn partial_cmp(&self, vector: &Self) -> Option<Ordering> {
        if utility::approx_eq(self.x, vector.x) {
            if utility::approx_eq(self.y, vector.y) {
                Some(Ordering::Equal)
            } else {
                self.y.partial_cmp(&vector.y)
            }
        } else {
            self.x.partial_cmp(&vector.x)
        }
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;
    fn add(self, vector: &Vector) -> Self::Output {
        Vector {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }
}
impl Sub<&Vector> for &Vector {
    type Output = Vector;
    fn sub(self, vector: &Vector) -> Self::Output {
        Vector {
            x: self.x - vector.x,
            y: self.y - vector.y,
        }
    }
}
impl Mul<f64> for &Vector {
    type Output = Vector;
    fn mul(self, factor: f64) -> Self::Output {
        Vector {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}
impl Mul<&Vector> for f64 {
    type Output = Vector;
    fn mul(self, vector: &Vector) -> Self::Output {
        Vector {
            x: self * vector.x,
            y: self * vector.y,
        }
    }
}
impl Div<f64> for &Vector {
    type Output = Vector;
    fn div(self, factor: f64) -> Self::Output {
        Vector {
            x: self.x / factor,
            y: self.y / factor,
        }
    }
}

impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, vector: &Vector) -> () {
        self.x += vector.x;
        self.y += vector.y;
    }
}
impl SubAssign<&Vector> for Vector {
    fn sub_assign(&mut self, vector: &Vector) -> () {
        self.x -= vector.x;
        self.y -= vector.y;
    }
}
impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, factor: f64) -> () {
        self.x *= factor;
        self.y *= factor;
    }
}
impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, factor: f64) -> () {
        self.x /= factor;
        self.y /= factor;
    }
}
