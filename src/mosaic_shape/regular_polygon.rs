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
        let radius = image_size.0.min(image_size.1) as f64 * scale;
        helpers::set_up_polygon_points(self.corners_count, radius, center_point, rotation_angle)
    }
    fn connect_points(&self, shape_points: &Vec<Vector>) -> Vec<Segment> {
        let points_count = shape_points.len();
        let mut segments = Vec::with_capacity(points_count * (points_count - 1) / 2);
        for start_index in 0..points_count - 1 {
            for end_index in start_index..points_count {
                segments.push(Segment::new(
                    shape_points[start_index].clone(),
                    shape_points[end_index].clone(),
                ));
            }
        }
        segments
    }
}
