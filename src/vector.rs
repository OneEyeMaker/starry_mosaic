use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use robust::Coord;
use voronoice::Point;

use super::utility;

/// Represents 2D vector.
///
/// Also represents 2D point because a point is result of moving origin by vector
/// and so has similar properties.
///
/// # Examples
///
/// This type provides implementation for different mathematical operations with vectors.
///
/// ```
/// use starry_mosaic::Vector;
///
/// let first_vector = Vector::new(1.0, -4.0);
/// let second_vector = Vector::new(3.0, 5.0);
/// let sum = first_vector + second_vector;
///
/// assert_eq!(sum, Vector::new(4.0, 1.0));
/// assert_eq!(sum - first_vector, second_vector);
///
/// let scaled_sum = 4.0 * sum;
///
/// assert_eq!(scaled_sum, Vector::new(16.0, 4.0));
/// assert_eq!(scaled_sum / 4.0, sum);
/// ```
///
/// Comparison of vectors takes into account the error of floating point calculations.
///
/// ```
/// use starry_mosaic::Vector;
///
/// let vector = Vector::new(5.0, 2.5);
/// let similar_vector = Vector::new(5.0 + f64::EPSILON * 4.0, 2.5 - f64::EPSILON * 2.0);
///
/// assert_eq!(vector, similar_vector);
/// ```
#[derive(Clone, Copy, Default)]
pub struct Vector {
    /// X coordinate (abscissa) of vector.
    pub x: f64,

    /// Y coordinate (ordinate) of vector.
    pub y: f64,
}

impl Vector {
    /// Builds 2D vector from its coordinates.
    #[inline(always)]
    pub fn new(x: f64, y: f64) -> Self {
        Vector { x, y }
    }

    /// Calculates squared length (squared magnitude) of vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::Vector;
    ///
    /// let vector = Vector::new(3.0, 4.0);
    ///
    /// assert_eq!(vector.squared_length(), 25.0);
    /// ```
    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// Calculates length (magnitude) of vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::Vector;
    ///
    /// let vector = Vector::new(3.0, 4.0);
    ///
    /// assert_eq!(vector.length(), 5.0);
    /// ```
    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    /// Finds squared distance from this to another point.
    ///
    /// # Arguments
    ///
    /// * `point`: point to which squared distance is calculated.
    ///
    /// returns: f64 - squared distance between this and another point.
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::Vector;
    ///
    /// let start_point = Vector::new(-1.0, 7.0);
    /// let end_point = Vector::new(4.0, -5.0);
    ///
    /// assert_eq!(start_point.squared_distance_to(end_point), 169.0);
    /// ```
    #[inline(always)]
    pub fn squared_distance_to(&self, point: Self) -> f64 {
        (*self - point).squared_length()
    }

    /// Finds distance from this to another point.
    ///
    /// # Arguments
    ///
    /// * `point`: point to which distance is calculated.
    ///
    /// returns: f64 - distance between this and another point.
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::Vector;
    ///
    /// let start_point = Vector::new(6.0, -7.0);
    /// let end_point = Vector::new(1.0, 5.0);
    ///
    /// assert_eq!(start_point.distance_to(end_point), 13.0);
    /// ```
    #[inline(always)]
    pub fn distance_to(&self, point: Self) -> f64 {
        (*self - point).length()
    }

    /// Creates normalized vector (one with same direction and magnitude of 1).
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::Vector;
    ///
    /// let vector = Vector::new(8.0, 6.0);
    ///
    /// assert_eq!(vector.get_normalized(), Vector::new(0.8, 0.6));
    /// ```
    pub fn get_normalized(&self) -> Self {
        *self / self.length()
    }

    /// Computes dot product of two vectors.
    ///
    /// # Arguments
    ///
    /// * `vector`: vector, operand of dot product.
    ///
    /// returns: f64 - dot product of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::Vector;
    ///
    /// let first_vector = Vector::new(2.5, -3.0);
    /// let second_vector = Vector::new(-4.0, 7.0);
    ///
    /// assert_eq!(first_vector.dot(second_vector), -31.0);
    /// assert_eq!(first_vector.dot(second_vector), second_vector.dot(first_vector));
    /// ```
    pub fn dot(&self, vector: Self) -> f64 {
        self.x * vector.x + self.y * vector.y
    }

    /// Computes difference between products of opposite coordinates of two vectors.
    ///
    /// Named so because algorithm is similar to one of cross product of 3D vectors.
    ///
    /// # Arguments
    ///
    /// * `vector`: vector, second operand of cross product.
    ///
    /// returns: f64 - difference between products of opposite coordinates of vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::Vector;
    ///
    /// let source_vector = Vector::new(2.0, -4.5);
    /// let target_vector = Vector::new(-1.5, 3.0);
    ///
    /// assert_eq!(source_vector.cross(target_vector), 0.75);
    /// ```
    pub fn cross(&self, vector: Self) -> f64 {
        self.y * vector.x - self.x * vector.y
    }

    /// Calculates linear interpolation between two vectors or points.
    ///
    /// # Arguments
    ///
    /// * `vector`: point (or vector) with which current point (vector) is interpolated.
    /// * `factor`: interpolation factor ranging from 0.0 to 1.0.
    ///
    /// returns: [`Vector`] - result of linear interpolation between two points or vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::Vector;
    ///
    /// let start_point = Vector::new(-2.0, 3.0);
    /// let end_point = Vector::new(7.0, -1.0);
    /// let interpolated_point = start_point.interpolate(end_point, 0.4);
    ///
    /// assert_eq!(interpolated_point, Vector::new(1.6, 1.4));
    /// ```
    pub fn interpolate(&self, vector: Self, factor: f64) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        Self {
            x: self.x + (vector.x - self.x) * factor,
            y: self.y + (vector.y - self.y) * factor,
        }
    }

    /// Translates current point by vector.
    ///
    /// # Arguments
    ///
    /// * `vector`: translation vector.
    ///
    /// returns: [`Vector`] - point resulting from translation (movement) of current point
    /// by vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::Vector;
    ///
    /// let point = Vector::new(-5.0, 7.0);
    /// let translated_point = point.translate(Vector::new(2.0, -3.0));
    ///
    /// assert_eq!(translated_point, Vector::new(-3.0, 4.0));
    /// ```
    #[inline(always)]
    pub fn translate(&self, vector: Self) -> Self {
        *self + vector
    }

    /// Rotates current point around origin (0.0, 0.0).
    ///
    /// # Arguments
    ///
    /// * `angle`: rotation angle.
    ///
    /// returns: [`Vector`] - point resulting from rotation of current point by angle.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts;
    ///
    /// use starry_mosaic::Vector;
    ///
    /// let point = Vector::new(4.0 * 2.0f64.sqrt(), 4.0 * 2.0f64.sqrt());
    ///
    /// assert_eq!(point.rotate(consts::FRAC_PI_4), Vector::new(0.0, 8.0));
    /// ```
    pub fn rotate(&self, angle: f64) -> Self {
        let sine = angle.sin();
        let cosine = angle.cos();
        Self {
            x: self.x * cosine - self.y * sine,
            y: self.x * sine + self.y * cosine,
        }
    }

    /// Rotates current point around pivot point.
    ///
    /// # Arguments
    ///
    /// * `angle`: rotation angle.
    /// * `pivot`: pivot point around which rotation is performed.
    ///
    /// returns: [`Vector`] - point resulting from rotation of current point around pivot point
    /// by angle.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts;
    ///
    /// use starry_mosaic::Vector;
    ///
    /// let point = Vector::new(4.0 * 2.0f64.sqrt() - 1.0, 4.0 * 2.0f64.sqrt() - 1.0);
    /// let pivot_point = Vector::new(-1.0, -1.0);
    ///
    /// assert_eq!(
    ///     point.rotate_around_pivot(consts::FRAC_PI_4, pivot_point),
    ///     Vector::new(-1.0, 7.0)
    /// );
    /// ```
    #[inline(always)]
    pub fn rotate_around_pivot(&self, angle: f64, pivot: Self) -> Self {
        (*self - pivot).rotate(angle) + pivot
    }

    /// Scales current vector by specified factors.
    ///
    /// # Arguments
    ///
    /// * `horizontal_scale`: factor of scaling in direction of X axis.
    /// * `vertical_scale`: factor of scaling in direction of Y axis.
    ///
    /// returns: [`Vector`] - vector resulting from scaling of current point by specified
    /// horizontal and vertical factors.
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::Vector;
    ///
    /// let vector = Vector::new(8.0, -2.0);
    /// let scaled_vector = vector.scale(0.5, 2.0);
    ///
    /// assert_eq!(scaled_vector, Vector::new(4.0, -4.0));
    /// ```
    #[inline(always)]
    pub fn scale(&self, horizontal_scale: f64, vertical_scale: f64) -> Self {
        *self * (horizontal_scale, vertical_scale)
    }

    /// Shears current point by specified factors.
    ///
    /// # Arguments
    ///
    /// * `horizontal_shear`: factor of shearing in direction of X axis.
    /// * `vertical_shear`: factor of shearing in direction of Y axis.
    ///
    /// returns: [`Vector`] - point resulting from shearing (skewing) of current point by specified
    /// horizontal and vertical factors.
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::Vector;
    ///
    /// let point = Vector::new(4.0, -2.0);
    /// let sheared_point = point.shear(-0.5, 0.25);
    ///
    /// assert_eq!(sheared_point, Vector::new(5.0, -1.0));
    /// ```
    pub fn shear(&self, horizontal_shear: f64, vertical_shear: f64) -> Self {
        Self {
            x: self.x + horizontal_shear * self.y,
            y: self.y + vertical_shear * self.x,
        }
    }

    pub(crate) fn round_to_epsilon(&self) -> Self {
        Self {
            x: utility::round_to_epsilon(self.x),
            y: utility::round_to_epsilon(self.y),
        }
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
impl From<Coord<f64>> for Vector {
    fn from(coord: Coord<f64>) -> Self {
        Self {
            x: coord.x,
            y: coord.y,
        }
    }
}
impl From<&Point> for Vector {
    fn from(point: &Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}
impl From<Vector> for Coord<f64> {
    fn from(vector: Vector) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
        }
    }
}
impl From<Vector> for Point {
    fn from(vector: Vector) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
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

impl Add for Vector {
    type Output = Vector;
    fn add(self, vector: Vector) -> Self::Output {
        Vector {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }
}
impl Sub for Vector {
    type Output = Vector;
    fn sub(self, vector: Vector) -> Self::Output {
        Vector {
            x: self.x - vector.x,
            y: self.y - vector.y,
        }
    }
}
impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, scale: f64) -> Self::Output {
        Vector {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}
impl Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, vector: Vector) -> Self::Output {
        Vector {
            x: self * vector.x,
            y: self * vector.y,
        }
    }
}
impl Mul<(f64, f64)> for Vector {
    type Output = Vector;
    fn mul(self, scale: (f64, f64)) -> Self::Output {
        Vector {
            x: self.x * scale.0,
            y: self.y * scale.1,
        }
    }
}
impl Mul<Vector> for (f64, f64) {
    type Output = Vector;
    fn mul(self, vector: Vector) -> Self::Output {
        Vector {
            x: self.0 * vector.x,
            y: self.1 * vector.y,
        }
    }
}
impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, scale: f64) -> Self::Output {
        Vector {
            x: self.x / scale,
            y: self.y / scale,
        }
    }
}
impl Div<(f64, f64)> for Vector {
    type Output = Vector;
    fn div(self, scale: (f64, f64)) -> Self::Output {
        Vector {
            x: self.x / scale.0,
            y: self.y / scale.1,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, vector: Vector) {
        self.x += vector.x;
        self.y += vector.y;
    }
}
impl SubAssign for Vector {
    fn sub_assign(&mut self, vector: Vector) {
        self.x -= vector.x;
        self.y -= vector.y;
    }
}
impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, scale: f64) {
        self.x *= scale;
        self.y *= scale;
    }
}
impl MulAssign<(f64, f64)> for Vector {
    fn mul_assign(&mut self, scale: (f64, f64)) {
        self.x *= scale.0;
        self.y *= scale.1;
    }
}
impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, scale: f64) {
        self.x /= scale;
        self.y /= scale;
    }
}
impl DivAssign<(f64, f64)> for Vector {
    fn div_assign(&mut self, scale: (f64, f64)) {
        self.x /= scale.0;
        self.y /= scale.1;
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts;

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
        assert_eq!(first.dot(second), 22.0);
    }
    #[test]
    fn dot_self_is_squared_length() {
        let vector = Vector::new(7.5, 4.0);
        assert_eq!(vector.dot(vector), vector.squared_length());
    }
    #[test]
    fn cross() {
        let first = Vector::new(5.0, 4.0);
        let second = Vector::new(3.0, 2.0);
        assert_eq!(first.cross(second), 2.0);
    }
    #[test]
    fn interpolate() {
        let first = Vector::new(5.0, 6.0);
        let second = Vector::new(1.0, -2.0);
        let interpolation = first.interpolate(second, 0.25);
        assert_eq!(interpolation.x, 4.0);
        assert_eq!(interpolation.y, 4.0);
    }
    #[test]
    fn translate() {
        let point = Vector::new(7.0, -2.0);
        let translated_point = point.translate(Vector::new(3.0, 3.0));
        assert_eq!(translated_point, Vector::new(10.0, 1.0));
    }
    #[test]
    fn rotate() {
        let vector = Vector::new(4.0, 0.0);
        assert_eq!(vector.rotate(consts::FRAC_PI_2), Vector::new(0.0, 4.0));
        assert_eq!(
            vector.rotate(consts::FRAC_PI_3),
            Vector::new(2.0, 2.0 * 3.0f64.sqrt())
        );
        assert_eq!(
            vector.rotate(consts::FRAC_PI_6),
            Vector::new(2.0 * 3.0f64.sqrt(), 2.0)
        );
    }
    #[test]
    fn rotate_around_pivot() {
        let vector = Vector::new(5.0, 2.0);
        let pivot = Vector::new(1.0, 2.0);
        assert_eq!(
            vector.rotate_around_pivot(consts::FRAC_PI_2, pivot),
            Vector::new(1.0, 6.0)
        );
        assert_eq!(
            vector.rotate_around_pivot(consts::FRAC_PI_3, pivot),
            Vector::new(3.0, 2.0 + 2.0 * 3.0f64.sqrt())
        );
        assert_eq!(
            vector.rotate_around_pivot(consts::FRAC_PI_6, pivot),
            Vector::new(1.0 + 2.0 * 3.0f64.sqrt(), 4.0)
        );
    }
    #[test]
    fn scale() {
        let vector = Vector::new(2.5, 5.0);
        let scaled_vector = vector.scale(2.0, 0.5);
        assert_eq!(scaled_vector, Vector::new(5.0, 2.5));
    }
    #[test]
    fn shear() {
        let point = Vector::new(-7.0, 3.0);
        let sheared_point = point.shear(1.0, -0.5);
        assert_eq!(sheared_point, Vector::new(-4.0, 6.5));
    }
    #[test]
    fn round_to_epsilon() {
        let vector = Vector::new(5.0 - f64::EPSILON * 2.0, -2.0 + f64::EPSILON * 4.0);
        let rounded_vector = vector.round_to_epsilon();
        assert_eq!(rounded_vector.x, 5.0);
        assert_eq!(rounded_vector.y, -2.0);
    }
    #[test]
    fn add() {
        let first = Vector::new(4.0, 5.0);
        let second = Vector::new(2.0, 3.0);
        let sum = first + second;
        assert_eq!(sum.x, 6.0);
        assert_eq!(sum.y, 8.0);
    }
    #[test]
    fn sub() {
        let first = Vector::new(4.0, 5.0);
        let second = Vector::new(2.0, 3.0);
        let difference = first - second;
        assert_eq!(difference.x, 2.0);
        assert_eq!(difference.y, 2.0);
    }
    #[test]
    fn mul() {
        let vector = Vector::new(2.0, 4.0);
        let multiplied = vector * 2.5;
        assert_eq!(multiplied.x, 5.0);
        assert_eq!(multiplied.y, 10.0);
    }
    #[test]
    fn mul_transitive() {
        let vector = Vector::new(2.0, 4.0);
        let first_multiplied = vector * 2.5;
        let second_multiplied = 2.5 * vector;
        assert_eq!(first_multiplied.x, second_multiplied.x);
        assert_eq!(first_multiplied.y, second_multiplied.y);
    }
    #[test]
    fn mul_tuple() {
        let vector = Vector::new(4.0, 1.0);
        let multiplied = vector * (0.5, 2.0);
        assert_eq!(multiplied.x, 2.0);
        assert_eq!(multiplied.y, 2.0);
    }
    #[test]
    fn mul_tuple_transitive() {
        let vector = Vector::new(6.0, 3.0);
        let first_multiplied = vector * (0.25, -0.5);
        let second_multiplied = (0.25, -0.5) * vector;
        assert_eq!(first_multiplied.x, second_multiplied.x);
        assert_eq!(first_multiplied.y, second_multiplied.y);
    }
    #[test]
    fn div() {
        let vector = Vector::new(4.0, 2.0);
        let divided = vector / 2.0;
        assert_eq!(divided.x, 2.0);
        assert_eq!(divided.y, 1.0);
    }
    #[test]
    fn div_tuple() {
        let vector = Vector::new(-8.0, 6.0);
        let divided = vector / (4.0, -3.0);
        assert_eq!(divided.x, -2.0);
        assert_eq!(divided.y, -2.0);
    }
    #[test]
    fn neg() {
        let vector = Vector::new(1.0, -1.0);
        let inverse_vector = -vector;
        assert_eq!(inverse_vector.x, -1.0);
        assert_eq!(inverse_vector.y, 1.0);
    }
    #[test]
    fn add_assign() {
        let mut vector = Vector::new(1.0, -2.0);
        vector += Vector::new(-5.0, 3.0);
        assert_eq!(vector.x, -4.0);
        assert_eq!(vector.y, 1.0);
    }
    #[test]
    fn sub_assign() {
        let mut vector = Vector::new(1.0, -2.0);
        vector -= Vector::new(-5.0, 3.0);
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
    fn mul_assign_tuple() {
        let mut vector = Vector::new(1.25, 4.5);
        vector *= (4.0, -2.0);
        assert_eq!(vector.x, 5.0);
        assert_eq!(vector.y, -9.0);
    }
    #[test]
    fn div_assign() {
        let mut vector = Vector::new(3.25, 2.5);
        vector /= 0.5;
        assert_eq!(vector.x, 6.5);
        assert_eq!(vector.y, 5.0);
    }
    #[test]
    fn div_assign_tuple() {
        let mut vector = Vector::new(1.25, 1.5);
        vector /= (0.5, 0.25);
        assert_eq!(vector.x, 2.5);
        assert_eq!(vector.y, 6.0);
    }
}
