use std::f64::consts;

use super::{MosaicShape, PolygonalShape, Segment, Vector};

#[derive(Clone, Debug)]
pub struct PolygonalStar {
    corners_count: usize,
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
        let additional_rotation_angle = consts::PI / corners_count;
        let additional_scale = (consts::PI * (corners_count * 0.5 - 2.0) / corners_count).sin()
            / (consts::FRAC_PI_2 / corners_count * (corners_count - 2.0)).sin();
        let mut points =
            self.calculate_polygon_points(image_size, center_point.clone(), rotation_angle, scale);
        let mut inner_points = self.calculate_polygon_points(
            image_size,
            center_point,
            rotation_angle + additional_rotation_angle,
            scale * additional_scale,
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

impl PolygonalShape for PolygonalStar {
    #[inline(always)]
    fn corners_count(&self) -> usize {
        self.corners_count
    }
}
