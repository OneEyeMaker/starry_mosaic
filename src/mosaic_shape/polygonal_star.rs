use std::f64::consts;

use super::{helpers, MosaicShape, Segment, Vector};

/// Defines mosaic shape based on polygonal star.
#[derive(Clone, Debug)]
pub struct PolygonalStar {
    corners_count: u32,
}

impl PolygonalStar {
    /// Creates polygonal star with set number of corners.
    ///
    /// # Arguments
    ///
    /// * `corners_count`: number of convex star corners; should be at least 3.
    ///
    /// returns: [`PolygonalStar`] - mosaic shape based on polygonal star with given
    /// number of convex corners.
    ///
    pub fn new(corners_count: u32) -> Self {
        Self {
            corners_count: corners_count.max(3),
        }
    }

    /// Number of convex corners of polygonal star on which mosaic shape is based.
    #[inline(always)]
    pub fn corners_count(&self) -> u32 {
        self.corners_count
    }

    /// Sets number of convex corners of polygonal star on which mosaic shape is based.
    ///
    /// # Arguments
    ///
    /// * `corners_count`: number of convex star corners; should be at least 3.
    ///
    /// returns: [`PolygonalStar`] - mosaic shape based on polygonal star with given
    /// number of convex corners.
    ///
    pub fn set_corners_count(&mut self, corners_count: u32) {
        self.corners_count = corners_count.max(3);
    }
}

impl Default for PolygonalStar {
    fn default() -> Self {
        Self { corners_count: 8 }
    }
}

impl MosaicShape for PolygonalStar {
    fn set_up_points(&self, image_width: u32, image_height: u32) -> Vec<Vector> {
        let corners_count = self.corners_count as f64;
        let radius = image_width.min(image_height) as f64 * 0.5;
        let inner_rotation_angle = consts::PI / corners_count;
        let inner_radius = radius
            * (consts::PI * (corners_count * 0.5 - 2.0) / corners_count).sin()
            / (consts::FRAC_PI_2 * (corners_count - 2.0) / corners_count).sin();
        let mut points = helpers::set_up_polygon_points(self.corners_count, radius, 0.0);
        let mut inner_points =
            helpers::set_up_polygon_points(self.corners_count, inner_radius, inner_rotation_angle);
        points.append(&mut inner_points);
        points
    }

    fn connect_points(&self, shape_points: &Vec<Vector>) -> Vec<Segment> {
        let points_count = shape_points.len() / 2;
        let mut segments = Vec::new();
        for start_index in 0..points_count {
            let end_index = (start_index + 2) % points_count;
            segments.push(Segment::new(
                shape_points[start_index],
                shape_points[end_index],
            ));
        }
        for start_index in 0..points_count {
            for end_index in start_index + 2..start_index + points_count - 2 {
                let end_index = points_count + end_index % points_count;
                segments.push(Segment::new(
                    shape_points[start_index],
                    shape_points[end_index],
                ));
            }
        }
        segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_corners_count() {
        let mut star = PolygonalStar::default();
        star.set_corners_count(12);
        assert_eq!(star.corners_count, 12);
    }
    #[test]
    fn set_incorrect_corners_count() {
        let mut star = PolygonalStar::default();
        star.set_corners_count(1);
        assert_eq!(star.corners_count, 3);
    }
    #[test]
    fn set_up_points() {
        let star = PolygonalStar::new(4);
        let points = star.set_up_points(400, 400);
        assert_eq!(points.len(), 8);
        for index in 0..4 {
            let angle = consts::FRAC_PI_2 * (index as f64 - 0.5);
            assert_eq!(
                points[index],
                Vector::new(200.0 * angle.cos(), 200.0 * angle.sin())
            );
        }
        for index in 4..8 {
            assert_eq!(points[index], Vector::new(0.0, 0.0));
        }
    }
    #[test]
    fn connect_points() {
        let star = PolygonalStar::new(4);
        let points = star.set_up_points(400, 400);
        let segments = star.connect_points(&points);
        let segment = Segment::from((
            (100.0 * 2.0f64.sqrt(), 100.0 * 2.0f64.sqrt()),
            (-100.0 * 2.0f64.sqrt(), -100.0 * 2.0f64.sqrt()),
        ));
        assert!(segments.contains(&segment));
        let segment = Segment::from((
            (100.0 * 2.0f64.sqrt(), -100.0 * 2.0f64.sqrt()),
            (-100.0 * 2.0f64.sqrt(), 100.0 * 2.0f64.sqrt()),
        ));
        assert!(segments.contains(&segment));
    }
    #[test]
    fn connect_points_of_hexagonal_star() {
        let star = PolygonalStar::new(6);
        let points = star.set_up_points(400, 400);
        let segments = star.connect_points(&points);
        let segment = Segment::from((
            (100.0, -100.0 * 3.0f64.sqrt()),
            (100.0, 100.0 * 3.0f64.sqrt()),
        ));
        assert!(segments.contains(&segment));
        let segment = Segment::from(((-200.0, 0.0), (100.0, -100.0 * 3.0f64.sqrt())));
        assert!(segments.contains(&segment));
        let segment = Segment::from((
            (-100.0, -100.0 * 3.0f64.sqrt()),
            (100.0, 100.0 * 3.0f64.sqrt()),
        ));
        assert!(!segments.contains(&segment));
    }
    #[test]
    fn intersect_segments_with_even_corners_count() {
        let star = PolygonalStar::new(8);
        let points = star.set_up_points(400, 400);
        let segments = star.connect_points(&points);
        let intersections = star.intersect_segments(&segments);
        assert!(!intersections.contains(&Vector::new(0.0, 0.0)));
    }
    #[test]
    fn intersect_segments_with_odd_corners_count() {
        let star = PolygonalStar::new(7);
        let points = star.set_up_points(400, 400);
        let segments = star.connect_points(&points);
        let intersections = star.intersect_segments(&segments);
        assert!(intersections.contains(&Vector::new(0.0, 0.0)));
    }
}
