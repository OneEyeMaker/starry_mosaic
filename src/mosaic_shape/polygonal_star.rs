use std::f64::consts;

use super::{helpers, MosaicShape, Segment, Vector};

#[derive(Clone, Debug)]
pub struct PolygonalStar {
    corners_count: u32,
}

impl PolygonalStar {
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

impl Default for PolygonalStar {
    fn default() -> Self {
        Self { corners_count: 8 }
    }
}

impl MosaicShape for PolygonalStar {
    fn set_up_points(
        &self,
        image_size: (u32, u32),
        center_point: Vector,
        rotation_angle: f64,
        scale: f64,
    ) -> Vec<Vector> {
        let corners_count = self.corners_count as f64;
        let radius = image_size.0.min(image_size.1) as f64 * scale;
        let inner_rotation_angle = rotation_angle + consts::PI / corners_count;
        let inner_radius = radius
            * (consts::PI * (corners_count * 0.5 - 2.0) / corners_count).sin()
            / (consts::FRAC_PI_2 * (corners_count - 2.0) / corners_count).sin();
        let mut points = helpers::set_up_polygon_points(
            self.corners_count,
            radius,
            center_point.clone(),
            rotation_angle,
        );
        let mut inner_points = helpers::set_up_polygon_points(
            self.corners_count,
            inner_radius,
            center_point,
            inner_rotation_angle,
        );
        points.append(&mut inner_points);
        points
    }
    fn connect_points(&self, shape_points: &Vec<Vector>) -> Vec<Segment> {
        let points_count = shape_points.len() / 2;
        let mut segments = Vec::new();
        for start_index in 0..points_count {
            let end_index = (start_index + 2) % points_count;
            segments.push(Segment::new(
                shape_points[start_index].clone(),
                shape_points[end_index].clone(),
            ));
        }
        for start_index in 0..points_count {
            for end_index in start_index + 2..start_index + points_count - 2 {
                let end_index = points_count + end_index % points_count;
                segments.push(Segment::new(
                    shape_points[start_index].clone(),
                    shape_points[end_index].clone(),
                ));
            }
        }
        segments
    }
}
