use std::fmt::Debug;

use super::{segment::Segment, vector::Vector};

/// Describes and calculates shape (pattern) of mosaic image.
///
/// Any mosaic shape is defined by set of key points. This set is created in 3 steps:
///
/// 1. Setting up of basic key points using method [`MosaicShape::set_up_points`].
/// 2. Connecting these basic key points with line segments using method
/// [`MosaicShape::connect_points`].
/// 3. Constructing rest key points by intersecting line segments from step 2 using method
/// [`MosaicShape::intersect_segments`].
///
/// **_Note_**: structs which implement `MosaicShape` should *not* store any points or
/// line segments. Instead all necessary geometry should be calculated by request.
///
/// Implementers of `MosaicShape` trait are required to implement [`Clone`] and [`Debug`] traits.
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
    /// Sets up primary key points of mosaic shape.
    ///
    /// # Arguments
    ///
    /// * `image_size`: width and height of mosaic image to generate.
    /// * `center`: position of center of mosaic shape in generated image.
    /// * `rotation_angle`: rotation angle of mosaic shape (pattern), in radians.
    /// * `scale`: scale of mosaic shape.
    ///
    /// returns: `Vec<`[`Vector`]`>` - set of basic key points of mosaic shape that fits into
    /// size of image and positioned, rotated and scaled appropriately.
    ///
    /// # See also
    ///
    /// * [`MosaicShape`].
    ///
    fn set_up_points(
        &self,
        image_size: (u32, u32),
        center: Vector,
        rotation_angle: f64,
        scale: f64,
    ) -> Vec<Vector>;

    /// Connects primary key points with line segments to form mosaic shape.
    ///
    /// # Arguments
    ///
    /// * `shape_points`: set of primary key points of mosaic image.
    ///
    /// returns: `Vec<`[`Segment`]`>` - list of segments which form mosaic shape.
    ///
    /// # See also
    ///
    /// * [`MosaicShape`].
    ///
    fn connect_points(&self, shape_points: &Vec<Vector>) -> Vec<Segment>;

    /// Intersects line segments of mosaic shape to construct its rest key points.
    ///
    /// # Arguments
    ///
    /// * `shape_segments`: list of line segments of mosaic shape.
    ///
    /// returns: `Vec<`[`Vector`]`>` - list of rest key points that defines mosaic shape.
    ///
    /// # See also
    ///
    /// * [`MosaicShape`].
    ///
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

/// Helper trait that implements [`Clone`] for `Box<dyn` [`MosaicShape`]`>`.
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
