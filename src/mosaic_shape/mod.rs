use std::f64::consts;

use super::{segment::Segment, vector::Vector};

pub trait MosaicShape {
    fn set_up_points(
        &self,
        image_size: (u32, u32),
        center_point: Vector,
        rotation_angle: f64,
        scale: f64,
    ) -> Vec<Vector>;
    fn connect_points(&self, shape_points: &Vec<Vector>) -> Vec<Segment>;
    fn intersect_segments(&self, shape_segments: &Vec<Segment>) -> Vec<Vector> {
        let mut points = Vec::new();
        for (index, first_segment) in shape_segments.iter().enumerate() {
            for second_segment in shape_segments[index..].iter() {
                if let Some(point) = first_segment.intersect(second_segment) {
                    points.push(point);
                }
            }
        }
        points
    }
}

pub trait PolygonalShape: MosaicShape {
    fn corners_count(&self) -> usize;
    fn calculate_polygon_points(
        &self,
        image_size: (u32, u32),
        center_point: Vector,
        rotation_angle: f64,
        scale: f64,
    ) -> Vec<Vector> {
        let radius = image_size.0.min(image_size.1) as f64 * scale;
        let corners_count = self.corners_count();
        let mut points = Vec::new();
        for index in 0..corners_count {
            let angle = rotation_angle
                + consts::PI / corners_count as f64 * (2 * index + 1 - corners_count % 2) as f64
                - consts::FRAC_PI_2;
            points.push(Vector::new(
                center_point.x + radius * angle.cos(),
                center_point.y + radius * angle.sin(),
            ));
        }
        points
    }
}
