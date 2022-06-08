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

mod helpers;
mod polygonal_star;
mod regular_polygon;

pub use polygonal_star::PolygonalStar;
pub use regular_polygon::RegularPolygon;
