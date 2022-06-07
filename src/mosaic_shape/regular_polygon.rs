use super::{MosaicShape, PolygonalShape, Segment, Vector};

#[derive(Clone, Debug)]
pub struct RegularPolygon {
    corners_count: usize,
}

impl RegularPolygon {
    pub fn new(corners_count: usize) -> Self {
        Self {
            corners_count: corners_count.max(3),
        }
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
        self.calculate_polygon_points(image_size, center_point, rotation_angle, scale)
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

impl PolygonalShape for RegularPolygon {
    #[inline(always)]
    fn corners_count(&self) -> usize {
        self.corners_count
    }
}
