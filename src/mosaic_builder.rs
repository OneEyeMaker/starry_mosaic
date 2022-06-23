use std::cmp::Ordering;

use voronoice::{BoundingBox, Point, Voronoi, VoronoiBuilder};

use super::{
    mosaic::Mosaic,
    mosaic_shape::{MosaicShape, PolygonalStar, RegularPolygon},
    starry_mosaic::StarryMosaic,
    vector::Vector,
};

/// Builds different mosaic images from set of its properties.
///
/// # Examples
///
/// Next example creates [starry mosaic image][`StarryMosaic`].
///
/// Uncomment last lines to save created image to file.
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
    /// Sets shape of mosaic image to [regular polygon][`RegularPolygon`].
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

    /// Sets shape of mosaic image to [polygonal star][`PolygonalStar`].
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

    /// Sets mosaic shape with which mosaic image will be created.
    ///
    /// # Arguments
    ///
    /// * `shape`: [mosaic shape][`MosaicShape`] which will be drawn in image.
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

    /// Sets width and height of mosaic image.
    ///
    /// # Arguments
    ///
    /// * `width`: width of image, in pixels; should be non-zero.
    /// * `height`: height of image, in pixels; should be non-zero.
    ///
    /// returns: [`MosaicBuilder`] - builder with configured image size.
    ///
    pub fn set_image_size(mut self, width: u32, height: u32) -> Self {
        self.image_size = (width.max(1), height.max(1));
        self
    }

    /// Sets center (pivot) point of shape of mosaic image.
    ///
    /// # Arguments
    ///
    /// * `center`: position of center of mosaic shape in created image; should be within image
    /// bounds.
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

    /// Sets rotation angle of shape of mosaic image.
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

    /// Sets scale of shape of mosaic image.
    ///
    /// # Arguments
    ///
    /// * `scale`: scale of mosaic shape in created image; should be at least 0.001.
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

    /// Builds mosaic image based on Voronoi diagram with current configuration of builder
    /// using constructor function.
    ///
    /// **_Note_**: this method is intended for building custom implementations of [`Mosaic`] trait.
    /// For existing implementations use other `build` methods.
    ///
    /// # Arguments
    ///
    /// * `constructor`: constructor function of mosaic image; this function takes next arguments:
    ///     * instance of [Voronoi diagram][`Voronoi`],
    ///     * width and height of created image,
    ///     * center point of shape of mosaic image,
    ///     * rotation angle of shape of mosaic image, in radians,
    ///     * scale of shape of mosaic image,
    ///     * mosaic shape with which mosaic image will be created.
    ///
    /// returns: `Option<MosaicImage>` - mosaic image based on Voronoi diagram with current
    /// configuration. Due to the fact that not every mosaic shape can provide valid set of
    /// key points for Voronoi diagram this method returns `Option<MosaicImage>` instead of
    /// `MosaicImage`.
    ///
    pub fn build_from_voronoi<MosaicImage, Constructor>(
        self,
        constructor: Constructor,
    ) -> Option<MosaicImage>
    where
        MosaicImage: Mosaic,
        Constructor:
            FnOnce(Voronoi, (u32, u32), Vector, f64, f64, Box<dyn MosaicShape>) -> MosaicImage,
    {
        let points = self
            .construct_shape()
            .iter()
            .map(|point| point.into())
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

    /// Builds mosaic image based on set of key points of mosaic shape with current configuration
    /// of builder using constructor function.
    ///
    /// **_Note_**: this method is intended for building custom implementations of [`Mosaic`] trait.
    /// For existing implementations use other `build` methods.
    ///
    /// # Arguments
    ///
    /// * `constructor`: constructor function of mosaic image; this function takes next arguments:
    ///     * set of key points calculated by constructing mosaic shape,
    ///     * width and height of created image,
    ///     * center point of shape of mosaic image,
    ///     * rotation angle of shape of mosaic image, in radians,
    ///     * scale of shape of mosaic image,
    ///     * mosaic shape with which mosaic image will be created.
    ///
    /// returns: `Option<MosaicImage>` - mosaic image based on Voronoi diagram with current
    /// configuration. Due to the fact that not every mosaic shape can provide valid set of
    /// key points for Voronoi diagram this method returns `Option<MosaicImage>` instead of
    /// `MosaicImage`.
    ///
    pub fn build_from_key_points<MosaicImage, Constructor>(
        self,
        constructor: Constructor,
    ) -> MosaicImage
    where
        MosaicImage: Mosaic,
        Constructor:
            FnOnce(Vec<Vector>, (u32, u32), Vector, f64, f64, Box<dyn MosaicShape>) -> MosaicImage,
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
            self.center.clone(),
            self.rotation_angle,
            self.scale,
        );
        let shape_segments = self.shape.connect_points(&initial_points);
        let mut shape_points = self.shape.intersect_segments(&shape_segments);
        shape_points.append(&mut initial_points);
        shape_points.sort_by(|left, right| left.partial_cmp(right).unwrap_or(Ordering::Equal));
        shape_points.dedup();
        shape_points
    }
}

impl Default for MosaicBuilder {
    fn default() -> Self {
        Self {
            shape: Box::new(RegularPolygon::default()),
            image_size: (400, 400),
            center: Vector::new(200.0, 200.0),
            rotation_angle: 0.0,
            scale: 0.75,
        }
    }
}

impl<MosaicImage> From<&MosaicImage> for MosaicBuilder
where
    MosaicImage: Mosaic,
{
    fn from(mosaic: &MosaicImage) -> Self {
        Self {
            shape: mosaic.shape(),
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
