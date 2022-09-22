use std::cmp::Ordering;

use voronoice::{BoundingBox, Point, Voronoi, VoronoiBuilder};

use super::{
    mosaic::Mosaic, mosaic_shape::*, polygonal_mosaic::PolygonalMosaic,
    starry_mosaic::StarryMosaic, vector::Vector,
};

/// Builds different mosaics from set of its properties.
///
/// # Examples
///
/// Next example creates [starry mosaic][`StarryMosaic`].
///
/// Uncomment last lines to create mosaic image and save it to file.
///
/// ```
/// use palette::LinSrgb;
/// use starry_mosaic::{Mosaic, MosaicBuilder, Vector};
///
/// let starry_mosaic = MosaicBuilder::default()
///     .set_image_size(1024, 1024)
///     .set_center(Vector::new(512.0, 512.0))
///     .set_rotation_angle(22.5f64.to_radians())
///     .set_scale(0.5)
///     .build_star();
///
/// assert!(starry_mosaic.is_some());
///
/// let starry_mosaic = starry_mosaic.unwrap();
///
/// assert_eq!(starry_mosaic.image_size(), (1024, 1024));
/// assert_eq!(starry_mosaic.center(), Vector::new(512.0, 512.0));
/// assert_eq!(starry_mosaic.rotation_angle(), 22.5f64.to_radians());
/// assert_eq!(starry_mosaic.scale(), 0.5);
///
/// // let orange_image = starry_mosaic.draw(LinSrgb::new(1.0f64, 0.5, 0.0));
/// // let save_result = orange_image.save("target/orange_starry_mosaic.png");
/// // assert!(save_result.is_ok());
/// ```
#[derive(Clone)]
pub struct MosaicBuilder {
    shape: Box<dyn MosaicShape>,
    image_size: (u32, u32),
    center: Vector,
    rotation_angle: f64,
    scale: f64,
}

impl MosaicBuilder {
    /// Sets shape of mosaic to [regular polygon][`RegularPolygon`].
    ///
    /// # Arguments
    ///
    /// * `corners_count`: number of polygon corners; should be at least 3.
    ///
    /// returns: [`MosaicBuilder`] - builder with mosaic shape set to regular polygon.
    ///
    /// # See also
    ///
    /// * [`MosaicBuilder::set_shape`].
    /// * [`RegularPolygon::new`].
    ///
    pub fn set_regular_polygon_shape(mut self, corners_count: u32) -> Self {
        self.shape = Box::new(RegularPolygon::new(corners_count));
        self
    }

    /// Sets shape of mosaic to [polygonal star][`PolygonalStar`].
    ///
    /// # Arguments
    ///
    /// * `corners_count`: number of convex star corners; should be at least 3.
    ///
    /// returns: [`MosaicBuilder`] - builder with mosaic shape set to polygonal star.
    ///
    /// # See also
    ///
    /// * [`MosaicBuilder::set_shape`].
    /// * [`PolygonalStar::new`].
    ///
    pub fn set_polygonal_star_shape(mut self, corners_count: u32) -> Self {
        self.shape = Box::new(PolygonalStar::new(corners_count));
        self
    }

    /// Sets shape of mosaic to grid (implemented using [tilted grid][`PolygonalStar`]).
    ///
    /// # Arguments
    ///
    /// * `rows_count`: number of grid rows; should be at least 1.
    /// * `columns_count`: number of grid columns; should be at least 1.
    ///
    /// returns: [`MosaicBuilder`] - builder with mosaic shape set to grid.
    ///
    /// # See Also
    ///
    /// * [`MosaicBuilder::set_tilted_grid_shape`].
    /// * [`MosaicBuilder::set_shape`].
    /// * [`TiltedGrid::new`].
    ///
    pub fn set_grid_shape(mut self, rows_count: u32, columns_count: u32) -> Self {
        self.shape = Box::new(TiltedGrid::new(rows_count, columns_count));
        self
    }

    /// Sets shape of mosaic to [tilted grid][`PolygonalStar`].
    ///
    /// # Arguments
    ///
    /// * `rows_count`: number of grid rows; should be at least 1.
    /// * `columns_count`: number of grid columns; should be at least 1.
    /// * `horizontal_factor`: tilt factor along X axis; value of 1.0 shifts the grid by its width.
    /// * `vertical_factor`: tilt factor along Y axis; value of 1.0 shifts the grid by its height.
    ///
    /// returns: [`MosaicBuilder`] - builder with mosaic shape set to tilted grid.
    ///
    /// # See Also
    ///
    /// * [`MosaicBuilder::set_grid_shape`].
    /// * [`MosaicBuilder::set_shape`].
    /// * [`TiltedGrid::new`].
    ///
    pub fn set_tilted_grid_shape(
        mut self,
        rows_count: u32,
        columns_count: u32,
        horizontal_tilt_factor: f64,
        vertical_tilt_factor: f64,
    ) -> Self {
        self.shape = Box::new(
            TiltedGrid::new(rows_count, columns_count)
                .tilt(horizontal_tilt_factor, vertical_tilt_factor),
        );
        self
    }

    /// Sets mosaic shape with which mosaic will be created.
    ///
    /// # Arguments
    ///
    /// * `shape`: [mosaic shape][`MosaicShape`] which will be drawn in mosaic image.
    ///
    /// returns: [`MosaicBuilder`] - builder with configured mosaic shape.
    ///
    pub fn set_shape<Shape>(mut self, shape: Shape) -> Self
    where
        Shape: 'static + MosaicShape,
    {
        self.shape = Box::new(shape);
        self
    }

    /// Sets width and height of mosaic (and mosaic images one creates).
    ///
    /// # Arguments
    ///
    /// * `width`: width of mosaic, in pixels; should be non-zero.
    /// * `height`: height of mosaic, in pixels; should be non-zero.
    ///
    /// returns: [`MosaicBuilder`] - builder with configured image size.
    ///
    pub fn set_image_size(mut self, width: u32, height: u32) -> Self {
        self.image_size = (width.max(1), height.max(1));
        self
    }

    /// Sets center (pivot) point of shape of mosaic.
    ///
    /// # Arguments
    ///
    /// * `center`: position of center of mosaic shape in created mosaic; should be within bounds
    /// of mosaic.
    ///
    /// returns: [`MosaicBuilder`] - builder with configured center of mosaic shape.
    ///
    pub fn set_center(mut self, center: Vector) -> Self {
        self.center = Vector::new(
            center.x.clamp(0.0, self.image_size.0 as f64),
            center.y.clamp(0.0, self.image_size.1 as f64),
        );
        self
    }

    /// Sets rotation angle of shape of mosaic.
    ///
    /// # Arguments
    ///
    /// * `rotation_angle`: rotation angle of mosaic shape, in radians.
    ///
    /// returns: [`MosaicBuilder`] - builder with configured rotation of mosaic shape.
    ///
    pub fn set_rotation_angle(mut self, rotation_angle: f64) -> Self {
        self.rotation_angle = rotation_angle;
        self
    }

    /// Sets scale of shape of mosaic.
    ///
    /// # Arguments
    ///
    /// * `scale`: scale of mosaic shape in created images; should be at least 0.001.
    ///
    /// returns: [`MosaicBuilder`] - builder with configured scale of mosaic shape.
    ///
    pub fn set_scale(mut self, scale: f64) -> Self {
        self.scale = scale.max(0.001);
        self
    }

    /// Builds [starry mosaic][`StarryMosaic`] with current configuration of builder.
    ///
    /// `StarryMosaic` is based on Voronoi diagram. Due to the fact that not every mosaic shape
    /// can provide valid set of key points for Voronoi diagram this method returns
    /// `Option<StarryMosaic>` instead of `StarryMosaic`.
    ///
    /// # See also
    ///
    /// * [`MosaicBuilder::build_from_voronoi`].
    ///
    pub fn build_star(self) -> Option<StarryMosaic> {
        self.build_from_voronoi(StarryMosaic::new)
    }

    /// Builds [polygonal mosaic][`PolygonalMosaic`] with current configuration of builder.
    ///
    /// `PolygonalMosaic` is based on Delaunay triangulation. Due to the fact that not every
    /// mosaic shape can provide valid set of key points for Delaunay triangulation this method
    /// returns `Option<PolygonalMosaic>` instead of `PolygonalMosaic`.
    ///
    /// # See also
    ///
    /// * [`MosaicBuilder::build_from_voronoi`].
    ///
    pub fn build_polygon(self) -> Option<PolygonalMosaic> {
        self.build_from_voronoi(PolygonalMosaic::new)
    }

    /// Builds mosaic based on Voronoi diagram with current configuration of builder
    /// using constructor function.
    ///
    /// **_Note_**: this method is intended for building custom implementations of [`Mosaic`] trait.
    /// For existing implementations use other `build` methods.
    ///
    /// # Arguments
    ///
    /// * `constructor`: constructor function of mosaic; this function takes next arguments:
    ///     * instance of [Voronoi diagram][`Voronoi`],
    ///     * width and height of mosaic (and created images),
    ///     * center point of shape of mosaic,
    ///     * rotation angle of shape of mosaic, in radians,
    ///     * scale of shape of mosaic,
    ///     * mosaic shape with which mosaic images will be created.
    ///
    /// returns: `Option<MosaicImplementation>` - configured mosaic based on Voronoi diagram.
    /// Due to the fact that not every mosaic shape can provide valid set of key points
    /// for Voronoi diagram this method returns `Option<MosaicImplementation>` instead of
    /// `MosaicImplementation`.
    ///
    pub fn build_from_voronoi<MosaicImplementation, Constructor>(
        self,
        constructor: Constructor,
    ) -> Option<MosaicImplementation>
    where
        MosaicImplementation: Mosaic,
        Constructor: FnOnce(
            Voronoi,
            (u32, u32),
            Vector,
            f64,
            f64,
            Box<dyn MosaicShape>,
        ) -> MosaicImplementation,
    {
        let points = self
            .construct_shape()
            .iter()
            .map(|point| (*point).into())
            .collect();
        let (image_width, image_height) = (self.image_size.0 as f64, self.image_size.1 as f64);
        let center = Point {
            x: image_width / 2.0,
            y: image_height / 2.0,
        };
        let voronoi = VoronoiBuilder::default()
            .set_bounding_box(BoundingBox::new(center, image_width, image_height))
            .set_sites(points)
            .build();
        match voronoi {
            Some(voronoi) => Some(constructor(
                voronoi,
                self.image_size,
                self.center,
                self.rotation_angle,
                self.scale,
                self.shape,
            )),
            None => None,
        }
    }

    /// Builds mosaic based on set of key points of mosaic shape with current configuration
    /// of builder using constructor function.
    ///
    /// **_Note_**: this method is intended for building custom implementations of [`Mosaic`] trait.
    /// For existing implementations use other `build` methods.
    ///
    /// # Arguments
    ///
    /// * `constructor`: constructor function of mosaic; this function takes next arguments:
    ///     * set of key points calculated by constructing mosaic shape,
    ///     * width and height of mosaic (and created images),
    ///     * center point of shape of mosaic,
    ///     * rotation angle of shape of mosaic, in radians,
    ///     * scale of shape of mosaic,
    ///     * mosaic shape with which mosaic images will be created.
    ///
    /// returns: `Option<MosaicImplementation>` - configured mosaic based on set of key point
    /// of constructed mosaic shape.
    ///
    pub fn build_from_key_points<MosaicImplementation, Constructor>(
        self,
        constructor: Constructor,
    ) -> MosaicImplementation
    where
        MosaicImplementation: Mosaic,
        Constructor: FnOnce(
            Vec<Vector>,
            (u32, u32),
            Vector,
            f64,
            f64,
            Box<dyn MosaicShape>,
        ) -> MosaicImplementation,
    {
        let points = self.construct_shape();
        constructor(
            points,
            self.image_size,
            self.center,
            self.rotation_angle,
            self.scale,
            self.shape,
        )
    }

    fn construct_shape(&self) -> Vec<Vector> {
        let mut initial_points = self.shape.set_up_points(
            self.image_size,
            self.center,
            self.rotation_angle,
            self.scale,
        );
        let shape_segments = self.shape.connect_points(&initial_points);
        let mut shape_points = self.shape.intersect_segments(&shape_segments);
        shape_points.append(&mut initial_points);
        shape_points.sort_by(|left, right| left.partial_cmp(right).unwrap_or(Ordering::Equal));
        shape_points
            .iter_mut()
            .for_each(|point| *point = point.round_to_epsilon());
        shape_points.dedup();
        shape_points
    }
}

impl Default for MosaicBuilder {
    fn default() -> Self {
        Self {
            shape: Box::new(RegularPolygon::default()),
            image_size: (640, 640),
            center: Vector::new(320.0, 320.0),
            rotation_angle: 0.0,
            scale: 1.0,
        }
    }
}

impl<MosaicImplementation> From<&MosaicImplementation> for MosaicBuilder
where
    MosaicImplementation: Mosaic,
{
    fn from(mosaic: &MosaicImplementation) -> Self {
        Self {
            shape: mosaic.shape().clone(),
            image_size: mosaic.image_size(),
            center: mosaic.center(),
            rotation_angle: mosaic.rotation_angle(),
            scale: mosaic.scale(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts;

    use super::*;

    #[test]
    fn set_image_size() {
        let builder = MosaicBuilder::default().set_image_size(320, 640);
        assert_eq!(builder.image_size, (320, 640));
    }
    #[test]
    fn set_incorrect_image_size() {
        let builder = MosaicBuilder::default().set_image_size(0, 0);
        assert!(builder.image_size.0 > 0);
        assert!(builder.image_size.1 > 0);
    }
    #[test]
    fn set_center() {
        let builder = MosaicBuilder::default().set_center(Vector::new(320.0, 160.0));
        assert_eq!(builder.center, Vector::new(320.0, 160.0));
    }
    #[test]
    fn set_center_out_of_bounds() {
        let builder = MosaicBuilder::default().set_center(Vector::new(-320.0, 10240.0));
        assert_eq!(builder.center.x, 0.0);
        assert_eq!(builder.center.y, builder.image_size.1 as f64);
    }
    #[test]
    fn set_rotation() {
        let builder = MosaicBuilder::default().set_rotation_angle(consts::FRAC_PI_4);
        assert_eq!(builder.rotation_angle, consts::FRAC_PI_4);
    }
    #[test]
    fn set_scale() {
        let builder = MosaicBuilder::default().set_scale(1.25);
        assert_eq!(builder.scale, 1.25);
    }
    #[test]
    fn set_zero_scale() {
        let builder = MosaicBuilder::default().set_scale(0.0);
        assert!(builder.scale > 0.0);
    }
}
