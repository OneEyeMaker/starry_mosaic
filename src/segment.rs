use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use super::{utility, vector::Vector};

/// Represents 2D line segment.
///
/// # Examples
///
/// Order of points that bound line segment does *__not__* matter.
///
/// ```
/// use starry_mosaic::{Segment, Vector};
///
/// let start_point = Vector::new(6.4, -2.3);
/// let end_point = Vector::new(1.7, 7.8);
///
/// let segment = Segment::new(start_point, end_point);
/// let inverse_segment = Segment::new(end_point, start_point);
///
/// assert_eq!(segment, inverse_segment);
/// ```
#[derive(Clone, Default)]
pub struct Segment {
    /// [Point][`Vector`] that is the boundary of line segment.
    pub start: Vector,

    /// [Point][`Vector`] that is the boundary of line segment.
    pub end: Vector,
}

impl Segment {
    /// Build line segment from two points bounding it.
    ///
    /// Note that order of points does *__not__* matter.
    #[inline(always)]
    pub fn new(start: Vector, end: Vector) -> Self {
        Segment { start, end }
    }

    /// Calculates squared length of line segment.
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::{Segment, Vector};
    ///
    /// let segment = Segment::new(Vector::new(2.0, -7.0), Vector::new(7.0, 5.0));
    ///
    /// assert_eq!(segment.squared_length(), 169.0);
    /// ```
    pub fn squared_length(&self) -> f64 {
        self.start.squared_distance_to(self.end)
    }

    /// Calculates length of line segment.
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::{Segment, Vector};
    ///
    /// let segment = Segment::new(Vector::new(-2.0, 6.0), Vector::new(10.0, 1.0));
    ///
    /// assert_eq!(segment.length(), 13.0);
    /// ```
    pub fn length(&self) -> f64 {
        self.start.distance_to(self.end)
    }

    /// Computes point of intersection of this line segment with another one, if such point exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::{Segment, Vector};
    ///
    /// let first_segment = Segment::new(Vector::new(-2.0, 2.0), Vector::new(3.5, -3.5));
    /// let second_segment = Segment::new(Vector::new(-3.0, -1.5), Vector::new(6.0, 3.0));
    /// let intersection = first_segment.intersect(&second_segment);
    ///
    /// assert!(intersection.is_some());
    /// let point = intersection.unwrap();
    /// assert_eq!(point, Vector::new(0.0, 0.0));
    /// ```
    pub fn intersect(&self, segment: &Self) -> Option<Vector> {
        let self_vector = self.end - self.start;
        let segment_vector = segment.end - segment.start;
        let denominator = self_vector.cross(segment_vector);
        if !utility::approx_eq(denominator, 0.0) {
            let start_vector = self.start - segment.start;
            let numerator = segment_vector.cross(start_vector);
            let factor = numerator / denominator;
            if factor > 0.0 && factor < 1.0 {
                return Some(self.start.interpolate(self.end, factor));
            }
        }
        None
    }
}

impl Debug for Segment {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.write_str("[")?;
        Debug::fmt(&self.start, formatter)?;
        formatter.write_str(" - ")?;
        Debug::fmt(&self.end, formatter)?;
        formatter.write_str("]")
    }
}
impl Display for Segment {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.write_str("[")?;
        Display::fmt(&self.start, formatter)?;
        formatter.write_str(" - ")?;
        Display::fmt(&self.end, formatter)?;
        formatter.write_str("]")
    }
}

impl<VectorLike> From<(VectorLike, VectorLike)> for Segment
where
    VectorLike: Into<Vector>,
{
    fn from(tuple: (VectorLike, VectorLike)) -> Self {
        Self {
            start: tuple.0.into(),
            end: tuple.1.into(),
        }
    }
}

impl PartialEq for Segment {
    fn eq(&self, segment: &Self) -> bool {
        (self.start == segment.start && self.end == segment.end)
            || (self.start == segment.end && self.end == segment.start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squared_length() {
        let segment = Segment::from(((1.0, 1.0), (4.0, 5.0)));
        assert_eq!(segment.squared_length(), 25.0);
    }
    #[test]
    fn length() {
        let segment = Segment::from(((1.0, 1.0), (4.0, 5.0)));
        assert_eq!(segment.length(), 5.0);
    }
    #[test]
    fn intersect() {
        let first = Segment::from(((-1.0, -1.0), (2.0, 2.0)));
        let second = Segment::from(((-3.0, 3.0), (5.0, -5.0)));
        let intersection = first.intersect(&second);
        assert!(intersection.is_some());
        let intersection = intersection.unwrap();
        assert_eq!(intersection.x, 0.0);
        assert_eq!(intersection.y, 0.0);
    }
    #[test]
    fn intersect_parallel() {
        let first = Segment::from(((-1.0, -1.0), (-3.0, -1.0)));
        let second = Segment::from(((-1.0, 4.0), (-3.0, 4.0)));
        let intersection = first.intersect(&second);
        assert!(intersection.is_none());
    }
}
