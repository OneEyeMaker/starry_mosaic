use super::{MosaicShape, Segment, Vector};

#[derive(Clone, Debug)]
pub struct TiltedGrid {
    cells_count: (u32, u32),
    tilt_factor: Vector,
}

impl TiltedGrid {
    pub fn new(horizontal_cells_count: u32, vertical_cells_count: u32) -> Self {
        Self {
            cells_count: (horizontal_cells_count.max(1), vertical_cells_count.max(1)),
            tilt_factor: Vector::default(),
        }
    }

    #[inline(always)]
    pub fn horizontal_cells_count(&self) -> u32 {
        self.cells_count.0
    }

    #[inline(always)]
    pub fn vertical_cells_count(&self) -> u32 {
        self.cells_count.1
    }

    pub fn set_cells_count(&mut self, horizontal_cells_count: u32, vertical_cells_count: u32) {
        self.cells_count = (horizontal_cells_count.max(1), vertical_cells_count.max(1));
    }

    #[inline(always)]
    pub fn horizontal_tilt_factor(&self) -> f64 {
        self.tilt_factor.x
    }

    #[inline(always)]
    pub fn vertical_tilt_factor(&self) -> f64 {
        self.tilt_factor.y
    }

    pub fn tilt(mut self, horizontal_factor: f64, vertical_factor: f64) -> Self {
        self.tilt_factor = Vector::new(horizontal_factor, vertical_factor);
        self
    }
}

impl Default for TiltedGrid {
    fn default() -> Self {
        Self {
            cells_count: (4, 4),
            tilt_factor: Vector::default(),
        }
    }
}

impl MosaicShape for TiltedGrid {
    fn set_up_points(
        &self,
        image_size: (u32, u32),
        center: Vector,
        rotation_angle: f64,
        scale: f64,
    ) -> Vec<Vector> {
        let (scaled_image_width, scaled_image_height) =
            (image_size.0 as f64 * scale, image_size.1 as f64 * scale);
        let (horizontal_step_size, vertical_step_size) = (
            scaled_image_width / self.cells_count.0 as f64,
            scaled_image_height / self.cells_count.1 as f64,
        );
        let step_size = horizontal_step_size.min(vertical_step_size);
        let (horizontal_half_size, vertical_half_size) = (
            step_size * self.cells_count.0 as f64 * 0.5,
            step_size * self.cells_count.1 as f64 * 0.5,
        );
        let mut points = vec![];
        points.push(Vector::new(-horizontal_half_size, -vertical_half_size));
        points.push(Vector::new(-horizontal_half_size, vertical_half_size));
        points.push(Vector::new(horizontal_half_size, -vertical_half_size));
        points.push(Vector::new(horizontal_half_size, vertical_half_size));
        for index in 1..self.cells_count.0 {
            let index = index as f64;
            points.push(Vector::new(
                -horizontal_half_size + step_size * index,
                -vertical_half_size,
            ));
            points.push(Vector::new(
                -horizontal_half_size + step_size * index,
                vertical_half_size,
            ));
        }
        for index in 1..self.cells_count.1 {
            let index = index as f64;
            points.push(Vector::new(
                -horizontal_half_size,
                -vertical_half_size + step_size * index,
            ));
            points.push(Vector::new(
                horizontal_half_size,
                -vertical_half_size + step_size * index,
            ));
        }
        points
            .iter()
            .map(|point| {
                &point
                    .shear(self.tilt_factor.x, self.tilt_factor.y)
                    .rotate(rotation_angle)
                    + &center
            })
            .collect()
    }

    fn connect_points(&self, shape_points: &Vec<Vector>) -> Vec<Segment> {
        let mut segments = vec![];
        let points_count = shape_points.len();
        for index in (4..points_count).step_by(2) {
            segments.push(Segment::new(
                shape_points[index].clone(),
                shape_points[index + 1].clone(),
            ));
        }
        segments
    }
}
