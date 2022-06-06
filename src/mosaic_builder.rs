use std::cmp::Ordering;
use std::f64::consts;

use voronoice::{BoundingBox, Point, VoronoiBuilder};

use super::{segment::Segment, starry_mosaic::StarryMosaic, vector::Vector};

#[derive(Clone, Debug)]
pub struct MosaicBuilder {
    image_size: (u32, u32),
    points_count: usize,
    rotation_angle: f64,
    center_point: Vector,
    scale: f64,
    is_complex_pattern: bool,
}

impl MosaicBuilder {
    pub fn set_image_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.image_size = (width.max(1), height.max(1));
        self
    }
    pub fn set_points_count(&mut self, points_count: usize) -> &mut Self {
        self.points_count = points_count.max(3);
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
    pub fn set_simple_pattern(&mut self) -> &mut Self {
        self.is_complex_pattern = false;
        self
    }
    pub fn set_complex_pattern(&mut self) -> &mut Self {
        self.is_complex_pattern = true;
        self
    }
    pub fn build_starry(&self) -> Option<StarryMosaic> {
        let points = self.build_key_points();
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
    fn calculate_polygon_points(
        &self,
        additional_rotation: f64,
        additional_scale: f64,
    ) -> Vec<Vector> {
        let radius =
            self.image_size.0.min(self.image_size.1) as f64 * self.scale * additional_scale;
        let mut points = Vec::new();
        for index in 0..self.points_count {
            let angle = consts::PI / self.points_count as f64
                * (2 * index + 1 - self.points_count % 2) as f64
                - consts::FRAC_PI_2
                + additional_rotation;
            points.push(Vector::new(
                self.center_point.x + radius * angle.cos(),
                self.center_point.y + radius * angle.sin(),
            ));
        }
        points
    }
    fn create_simple_pattern_segments(&self, points: &Vec<Vector>) -> Vec<Segment> {
        let points_count = points.len();
        let mut segments = Vec::with_capacity(points_count * (points_count - 1) / 2);
        for start_index in 0..points_count - 1 {
            for end_index in start_index..points_count {
                segments.push(Segment::new(
                    points[start_index].clone(),
                    points[end_index].clone(),
                ));
            }
        }
        segments
    }
    fn create_complex_pattern_segments(
        &self,
        outer_points: &Vec<Vector>,
        inner_points: &Vec<Vector>,
    ) -> Vec<Segment> {
        debug_assert!(outer_points.len() == inner_points.len());
        let points_count = outer_points.len();
        let mut segments = Vec::new();
        for start_index in 0..points_count {
            let end_index = (start_index + 2) % points_count;
            segments.push(Segment::new(
                outer_points[start_index].clone(),
                outer_points[end_index].clone(),
            ));
        }
        for start_index in 0..points_count {
            for end_index in start_index + 2..start_index + points_count - 2 {
                let end_index = end_index % points_count;
                segments.push(Segment::new(
                    outer_points[start_index].clone(),
                    inner_points[end_index].clone(),
                ));
            }
        }
        segments
    }
    fn intersect_segments(&self, segments: &Vec<Segment>, points: &mut Vec<Vector>) {
        for (index, first_segment) in segments.iter().enumerate() {
            for second_segment in segments[index + 1..].iter() {
                if let Some(point) = first_segment.intersect(second_segment) {
                    points.push(point);
                }
            }
        }
    }
    fn build_key_points(&self) -> Vec<Point> {
        let mut points = self.calculate_polygon_points(0.0, 1.0);
        let segments = if self.is_complex_pattern {
            let points_count = self.points_count as f64;
            let additional_rotation = consts::PI / points_count;
            let additional_scale = (consts::PI * (points_count * 0.5 - 2.0) / points_count).sin()
                / (consts::FRAC_PI_2 / points_count * (points_count - 2.0)).sin();
            let mut inner_points =
                self.calculate_polygon_points(additional_rotation, additional_scale);
            let segments = self.create_complex_pattern_segments(&points, &inner_points);
            points.append(&mut inner_points);
            segments
        } else {
            self.create_simple_pattern_segments(&points)
        };
        self.intersect_segments(&segments, &mut points);
        points.sort_by(|left, right| left.partial_cmp(right).unwrap_or(Ordering::Equal));
        points.dedup();
        points.iter().map(|point| point.into()).collect()
    }
}

impl Default for MosaicBuilder {
    fn default() -> Self {
        Self {
            image_size: (400, 400),
            points_count: 8,
            rotation_angle: 0.0,
            center_point: Vector::new(200.0, 200.0),
            scale: 0.75,
            is_complex_pattern: false,
        }
    }
}
