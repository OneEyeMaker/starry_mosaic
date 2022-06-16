use std::fmt::Debug;

use super::{segment::Segment, vector::Vector};

pub trait MosaicShape: Debug + MosaicShapeBase {
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

pub trait MosaicShapeBase {
    fn clone_box(&self) -> Box<dyn MosaicShape>;
}

impl<T> MosaicShapeBase for T
where
    T: 'static + MosaicShape + Clone,
{
    fn clone_box(&self) -> Box<dyn MosaicShape> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn MosaicShape> {
    #[inline(always)]
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

mod helpers;
mod polygonal_star;
mod regular_polygon;

pub use polygonal_star::PolygonalStar;
pub use regular_polygon::RegularPolygon;
