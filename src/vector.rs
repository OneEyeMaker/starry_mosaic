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
    fn add_assign(&mut self, vector: &Vector) {
        self.x += vector.x;
        self.y += vector.y;
    }
}
impl SubAssign<&Vector> for Vector {
    fn sub_assign(&mut self, vector: &Vector) {
        self.x -= vector.x;
        self.y -= vector.y;
    }
}
impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }
}
impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, factor: f64) {
        self.x /= factor;
        self.y /= factor;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squared_length() {
        let vector = Vector::new(3.0, 4.0);
        assert_eq!(vector.squared_length(), 25.0);
    }
    #[test]
    fn length() {
        let vector = Vector::new(3.0, 4.0);
        assert_eq!(vector.length(), 5.0);
    }
    #[test]
    fn length_is_hypot() {
        let vector = Vector::new(8.0, 6.0);
        assert_eq!(vector.length(), vector.x.hypot(vector.y));
    }
    #[test]
    fn get_normalized() {
        let vector = Vector::new(8.0, 6.0).get_normalized();
        assert_eq!(vector.x, 0.8);
        assert_eq!(vector.y, 0.6);
    }
    #[test]
    fn dot() {
        let first = Vector::new(3.0, 5.0);
        let second = Vector::new(4.0, 2.0);
        assert_eq!(first.dot(&second), 22.0);
    }
    #[test]
    fn dot_self_is_squared_length() {
        let vector = Vector::new(7.5, 4.0);
        assert_eq!(vector.dot(&vector), vector.squared_length());
    }
    #[test]
    fn cross() {
        let first = Vector::new(5.0, 4.0);
        let second = Vector::new(3.0, 2.0);
        assert_eq!(first.cross(&second), 2.0);
    }
    #[test]
    fn add() {
        let first = Vector::new(4.0, 5.0);
        let second = Vector::new(2.0, 3.0);
        let sum = &first + &second;
        assert_eq!(sum.x, 6.0);
        assert_eq!(sum.y, 8.0);
    }
    #[test]
    fn sub() {
        let first = Vector::new(4.0, 5.0);
        let second = Vector::new(2.0, 3.0);
        let difference = &first - &second;
        assert_eq!(difference.x, 2.0);
        assert_eq!(difference.y, 2.0);
    }
    #[test]
    fn mul() {
        let vector = Vector::new(2.0, 4.0);
        let multiplied = &vector * 2.5;
        assert_eq!(multiplied.x, 5.0);
        assert_eq!(multiplied.y, 10.0);
    }
    #[test]
    fn mul_transitive() {
        let vector = Vector::new(2.0, 4.0);
        let first_multiplied = &vector * 2.5;
        let second_multiplied = 2.5 * &vector;
        assert_eq!(first_multiplied.x, second_multiplied.x);
        assert_eq!(first_multiplied.y, second_multiplied.y);
    }
    #[test]
    fn div() {
        let vector = Vector::new(4.0, 2.0);
        let divided = &vector / 2.0;
        assert_eq!(divided.x, 2.0);
        assert_eq!(divided.y, 1.0);
    }
    #[test]
    fn add_assign() {
        let mut vector = Vector::new(1.0, -2.0);
        vector += &Vector::new(-5.0, 3.0);
        assert_eq!(vector.x, -4.0);
        assert_eq!(vector.y, 1.0);
    }
    #[test]
    fn sub_assign() {
        let mut vector = Vector::new(1.0, -2.0);
        vector -= &Vector::new(-5.0, 3.0);
        assert_eq!(vector.x, 6.0);
        assert_eq!(vector.y, -5.0);
    }
    #[test]
    fn mul_assign() {
        let mut vector = Vector::new(3.25, 2.5);
        vector *= 4.0;
        assert_eq!(vector.x, 13.0);
        assert_eq!(vector.y, 10.0);
    }
    #[test]
    fn div_assign() {
        let mut vector = Vector::new(3.25, 2.5);
        vector /= 0.5;
        assert_eq!(vector.x, 6.5);
        assert_eq!(vector.y, 5.0);
    }
}
