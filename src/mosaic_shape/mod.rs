use std::fmt::Debug;

use super::{segment::Segment, vector::Vector};

/// Describes and calculates shape (pattern) of mosaic image.
///
/// # Examples
///
/// Next example implements grid shape with at least 2 columns and 2 rows.
///
/// ```
/// use std::fmt::Debug;
///
/// use starry_mosaic::{mosaic_shape::MosaicShape, Segment, Vector};
///
/// #[derive(Clone, Debug)]
/// struct GridShape {
///     partitions_count: usize,
/// }
/// impl GridShape {
///     fn new(partitions_count: usize) -> Self {
///         Self {
///             partitions_count: partitions_count.max(2),
///         }
///     }
/// }
/// impl MosaicShape for GridShape {
///     fn set_up_points(
///         &self,
///         image_size: (u32, u32),
///         center: Vector,
///         rotation_angle: f64,
///         scale: f64,
///     ) -> Vec<Vector> {
///         let (image_width, image_height) = image_size;
///         let size = image_width.min(image_height) as f64 * scale;
///         let half_size = size * 0.5;
///         let step_size = size / self.partitions_count as f64;
///         let mut points = vec![];
///         for index in 1..self.partitions_count {
///             let index = index as f64;
///             points.push(Vector::new(-half_size + step_size * index, -half_size));
///             points.push(Vector::new(-half_size + step_size * index, half_size));
///             points.push(Vector::new(-half_size, -half_size + step_size * index));
///             points.push(Vector::new(half_size, -half_size + step_size * index));
///         }
///         points
///             .iter()
///             .map(|point| &point.rotate(rotation_angle) + &center)
///             .collect()
///     }
///     fn connect_points(&self, shape_points: &Vec<Vector>) -> Vec<Segment> {
///         let mut segments = vec![];
///         let points_count = shape_points.len();
///         for index in (0..points_count).step_by(4) {
///             segments.push(Segment::new(
///                 shape_points[index].clone(),
///                 shape_points[index + 1].clone(),
///             ));
///             segments.push(Segment::new(
///                 shape_points[index + 2].clone(),
///                 shape_points[index + 3].clone(),
///             ));
///         }
///         segments
///     }
/// }
///
/// fn main() {
///     let grid = GridShape::new(4);
///     let points = grid.set_up_points((200, 200), Vector::new(100.0, 100.0), 0.0, 1.0);
///
///     assert_eq!(points.len(), (grid.partitions_count - 1) * 4);
///     assert!(points.contains(&Vector::new(50.0, 0.0)));
///     assert!(points.contains(&Vector::new(150.0, 200.0)));
///
///     let segments = grid.connect_points(&points);
///
///     let horizontal_segment = Segment::from(((0.0, 100.0), (200.0, 100.0)));
///     let vertical_segment = Segment::from(((100.0, 0.0), (100.0, 200.0)));
///     assert!(segments.contains(&horizontal_segment));
///     assert!(segments.contains(&vertical_segment));
/// }
/// ```
pub trait MosaicShape: Debug + MosaicShapeBase {
    fn set_up_points(
        &self,
        image_size: (u32, u32),
        center: Vector,
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
