use std::cmp::Ordering;

use voronoice::{BoundingBox, Point, VoronoiBuilder};

use super::{
    mosaic_shape::{MosaicShape, PolygonalStar, RegularPolygon},
    starry_mosaic::StarryMosaic,
    vector::Vector,
};

pub struct MosaicBuilder {
    shape: Box<dyn MosaicShape>,
    image_size: (u32, u32),
    center_point: Vector,
    rotation_angle: f64,
    scale: f64,
}

impl MosaicBuilder {
    pub fn set_regular_polygon_shape(&mut self, corners_count: usize) -> &mut Self {
        self.shape = Box::new(RegularPolygon::new(corners_count));
        self
    }
    pub fn set_polygonal_star_shape(&mut self, corners_count: usize) -> &mut Self {
        self.shape = Box::new(PolygonalStar::new(corners_count));
        self
    }
    pub fn set_shape<Shape>(&mut self, shape: Shape) -> &mut Self
    where
        Shape: MosaicShape + 'static,
    {
        self.shape = Box::new(shape);
        self
    }
    pub fn set_image_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.image_size = (width.max(1), height.max(1));
        self
    }
    pub fn set_center_point(&mut self, center_point: Vector) -> &mut Self {
        self.center_point = Vector::new(
            center_point.x.clamp(0.0, self.image_size.0 as f64),
            center_point.y.clamp(0.0, self.image_size.1 as f64),
        );
        self
    }
    pub fn set_rotation_angle(&mut self, rotation_angle: f64) -> &mut Self {
        self.rotation_angle = rotation_angle;
        self
    }
    pub fn set_scale(&mut self, scale: f64) -> &mut Self {
        self.scale = scale.max(0.001);
        self
    }
    pub fn build_starry(&self) -> Option<StarryMosaic> {
        let points = self.build_shape();
        let image_size = (self.image_size.0 as f64, self.image_size.1 as f64);
        let center = Point {
            x: image_size.0 / 2.0,
            y: image_size.1 / 2.0,
        };
        let voronoi = VoronoiBuilder::default()
            .set_bounding_box(BoundingBox::new(center, image_size.0, image_size.1))
            .set_sites(points)
            .build();
        match voronoi {
            Some(voronoi) => Some(StarryMosaic::new(voronoi, self.image_size)),
            None => None,
        }
    }
    fn build_shape(&self) -> Vec<Point> {
        let mut initial_points = self.shape.set_up_points(
            self.image_size,
            self.center_point.clone(),
            self.rotation_angle,
            self.scale,
        );
        let shape_segments = self.shape.connect_points(&initial_points);
        let mut shape_points = self.shape.intersect_segments(&shape_segments);
        shape_points.append(&mut initial_points);
        shape_points.sort_by(|left, right| left.partial_cmp(right).unwrap_or(Ordering::Equal));
        shape_points.dedup();
        shape_points.iter().map(|point| point.into()).collect()
    }
}

impl Default for MosaicBuilder {
    fn default() -> Self {
        Self {
            shape: Box::new(RegularPolygon::default()),
            image_size: (400, 400),
            center_point: Vector::new(200.0, 200.0),
            rotation_angle: 0.0,
            scale: 0.75,
        }
    }
}
