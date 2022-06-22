use std::cmp::Ordering;

use voronoice::{BoundingBox, Point, Voronoi, VoronoiBuilder};

use super::{
    mosaic::Mosaic,
    mosaic_shape::{MosaicShape, PolygonalStar, RegularPolygon},
    starry_mosaic::StarryMosaic,
    vector::Vector,
};

#[derive(Clone)]
pub struct MosaicBuilder {
    shape: Box<dyn MosaicShape>,
    image_size: (u32, u32),
    center: Vector,
    rotation_angle: f64,
    scale: f64,
}

impl MosaicBuilder {
    pub fn set_regular_polygon_shape(mut self, corners_count: u32) -> Self {
        self.shape = Box::new(RegularPolygon::new(corners_count));
        self
    }
    pub fn set_polygonal_star_shape(mut self, corners_count: u32) -> Self {
        self.shape = Box::new(PolygonalStar::new(corners_count));
        self
    }
    pub fn set_shape<Shape>(mut self, shape: Shape) -> Self
    where
        Shape: 'static + MosaicShape,
    {
        self.shape = Box::new(shape);
        self
    }
    pub fn set_image_size(mut self, width: u32, height: u32) -> Self {
        self.image_size = (width.max(1), height.max(1));
        self
    }
    pub fn set_center(mut self, center: Vector) -> Self {
        self.center = Vector::new(
            center.x.clamp(0.0, self.image_size.0 as f64),
            center.y.clamp(0.0, self.image_size.1 as f64),
        );
        self
    }
    pub fn set_rotation_angle(mut self, rotation_angle: f64) -> Self {
        self.rotation_angle = rotation_angle;
        self
    }
    pub fn set_scale(mut self, scale: f64) -> Self {
        self.scale = scale.max(0.001);
        self
    }
    pub fn build_star(self) -> Option<StarryMosaic> {
        self.build_from_voronoi(StarryMosaic::new)
    }
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
    pub fn build_from_key_points<MosaicImage, Constructor>(
        self,
        constructor: Constructor,
    ) -> Option<MosaicImage>
    where
        MosaicImage: Mosaic,
        Constructor: FnOnce(
            Vec<Vector>,
            (u32, u32),
            Vector,
            f64,
            f64,
            Box<dyn MosaicShape>,
        ) -> Option<MosaicImage>,
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
