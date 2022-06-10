use super::{helpers, MosaicShape, Segment, Vector};

#[derive(Clone, Debug)]
pub struct RegularPolygon {
    corners_count: u32,
}

impl RegularPolygon {
    pub fn new(corners_count: u32) -> Self {
        Self {
            corners_count: corners_count.max(3),
        }
    }
    #[inline(always)]
    pub fn corners_count(&self) -> u32 {
        self.corners_count
    }
    pub fn set_corners_count(&mut self, corners_count: u32) {
        self.corners_count = corners_count.max(3);
    }
}

impl Default for RegularPolygon {
    fn default() -> Self {
        Self { corners_count: 8 }
    }
}

impl MosaicShape for RegularPolygon {
    fn set_up_points(
        &self,
        image_size: (u32, u32),
        center_point: Vector,
        rotation_angle: f64,
        scale: f64,
    ) -> Vec<Vector> {
        let radius = image_size.0.min(image_size.1) as f64 * 0.5 * scale;
        helpers::set_up_polygon_points(self.corners_count, radius, center_point, rotation_angle)
    }
    fn connect_points(&self, shape_points: &Vec<Vector>) -> Vec<Segment> {
        let points_count = shape_points.len();
        let mut segments = Vec::with_capacity(points_count * (points_count - 1) / 2);
        for start_index in 0..points_count - 1 {
            for end_index in start_index + 1..points_count {
                segments.push(Segment::new(
                    shape_points[start_index].clone(),
                    shape_points[end_index].clone(),
                ));
            }
        }
        segments
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts;

    use super::*;

    #[test]
    fn set_corners_count() {
        let mut polygon = RegularPolygon::default();
        polygon.set_corners_count(12);
        assert_eq!(polygon.corners_count, 12);
    }
    #[test]
    fn set_incorrect_corners_count() {
        let mut polygon = RegularPolygon::default();
        polygon.set_corners_count(1);
        assert_eq!(polygon.corners_count, 3);
    }
    #[test]
    fn set_up_points() {
        let polygon = RegularPolygon::new(8);
        let points = polygon.set_up_points(
            (400, 400),
            Vector::new(200.0, 200.0),
            consts::FRAC_PI_8,
            0.5,
        );
        assert_eq!(points.len(), 8);
        for index in 0..8 {
            let angle = consts::FRAC_PI_4 * (index as f64 - 1.0);
            assert_eq!(
                points[index],
                Vector::new(200.0 + 100.0 * angle.cos(), 200.0 + 100.0 * angle.sin())
            );
        }
    }
    #[test]
    fn connect_points() {
        let polygon = RegularPolygon::new(8);
        let points = polygon.set_up_points(
            (400, 400),
            Vector::new(200.0, 200.0),
            consts::FRAC_PI_8,
            0.5,
        );
        let segments = polygon.connect_points(&points);
        assert_eq!(segments.len(), 28);
        let segment = Segment::from(((100.0, 200.0), (300.0, 200.0)));
        assert!(segments.contains(&segment));
        let segment = Segment::from(((200.0, 100.0), (200.0, 300.0)));
        assert!(segments.contains(&segment));
    }
    #[test]
    fn intersect_segments() {
        let polygon = RegularPolygon::new(8);
        let points = polygon.set_up_points(
            (400, 400),
            Vector::new(200.0, 200.0),
            consts::FRAC_PI_8,
            0.5,
        );
        let segments = polygon.connect_points(&points);
        let intersections = polygon.intersect_segments(&segments);
        assert!(intersections.contains(&Vector::new(200.0, 200.0)));
    }
}
