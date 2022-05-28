use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use super::{utility, vector::Vector};

#[derive(Clone, Default)]
pub struct Segment {
    pub start: Vector,
    pub end: Vector,
}

impl Segment {
    #[inline(always)]
    pub fn new(start: Vector, end: Vector) -> Self {
        Segment {
            start,
            end,
        }
    }
    pub fn squared_length(&self) -> f64 {
        self.start.squared_distance_to(&self.end)
    }
    pub fn length(&self) -> f64 {
        self.start.distance_to(&self.end)
    }
    pub fn intersect(&self, segment: &Self) -> Option<Vector> {
        let self_vector = &self.end - &self.start;
        let segment_vector = &segment.end - &segment.start;
        let denominator = self_vector.cross(&segment_vector);
        if !utility::approx_eq(denominator, 0.0) {
            let start_vector = &self.start - &segment.start;
            let numerator = segment_vector.cross(&start_vector);
            let factor = numerator / denominator;
            if factor > 0.0 && factor < 1.0 {
                return Some(&self.start + &(factor * &self_vector));
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
