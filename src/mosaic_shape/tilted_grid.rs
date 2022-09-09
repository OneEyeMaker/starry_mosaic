use super::{MosaicShape, Segment, Vector};

#[derive(Clone, Debug)]
pub struct TiltedGrid {
    cells_count: (u32, u32),
    tilt_factor: Vector,
}

impl TiltedGrid {
    pub fn new(rows_count: u32, columns_count: u32) -> Self {
        Self {
            cells_count: (rows_count.max(1), columns_count.max(1)),
            tilt_factor: Vector::default(),
        }
    }

    #[inline(always)]
    pub fn rows_count(&self) -> u32 {
        self.cells_count.0
    }

    #[inline(always)]
    pub fn columns_count(&self) -> u32 {
        self.cells_count.1
    }

    pub fn set_rows_count(&mut self, rows_count: u32) {
        self.cells_count.0 = rows_count.max(1);
    }

    pub fn set_columns_count(&mut self, columns_count: u32) {
        self.cells_count.1 = columns_count.max(1);
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
        let (rows_count, columns_count) = self.cells_count;
        let (horizontal_step_size, vertical_step_size) = (
            scaled_image_width / columns_count as f64,
            scaled_image_height / rows_count as f64,
        );
        let step_size = horizontal_step_size.min(vertical_step_size);
        let (horizontal_half_size, vertical_half_size) = (
            step_size * columns_count as f64 * 0.5,
            step_size * rows_count as f64 * 0.5,
        );
        let mut points = vec![];
        points.push(Vector::new(-horizontal_half_size, -vertical_half_size));
        points.push(Vector::new(-horizontal_half_size, vertical_half_size));
        points.push(Vector::new(horizontal_half_size, -vertical_half_size));
        points.push(Vector::new(horizontal_half_size, vertical_half_size));
        for index in 1..rows_count {
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
        for index in 1..columns_count {
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

#[cfg(test)]
mod tests {
    use std::f64::consts;

    use super::*;

    #[test]
    fn set_rows_count() {
        let mut grid = TiltedGrid::default();
        grid.set_rows_count(7);
        assert_eq!(grid.rows_count(), 7);
    }
    #[test]
    fn set_incorrect_rows_count() {
        let mut grid = TiltedGrid::default();
        grid.set_rows_count(0);
        assert_eq!(grid.rows_count(), 1);
    }
    #[test]
    fn set_columns_count() {
        let mut grid = TiltedGrid::default();
        grid.set_columns_count(15);
        assert_eq!(grid.columns_count(), 15);
    }
    #[test]
    fn set_incorrect_columns_count() {
        let mut grid = TiltedGrid::default();
        grid.set_columns_count(0);
        assert_eq!(grid.columns_count(), 1);
    }
    #[test]
    fn tilt() {
        let grid = TiltedGrid::default().tilt(0.25, -0.5);
        assert_eq!(grid.horizontal_tilt_factor(), 0.25);
        assert_eq!(grid.vertical_tilt_factor(), -0.5);
    }
    #[test]
    fn set_up_points() {
        let grid = TiltedGrid::new(4, 4).tilt(0.25, -0.5);
        let points = grid.set_up_points(
            (400, 400),
            Vector::new(200.0, 200.0),
            consts::FRAC_PI_2,
            0.5,
        );
        assert_eq!(points.len(), 16);
        assert!(points.contains(&Vector::new(250.0, 75.0)));
        assert!(points.contains(&Vector::new(150.0, 325.0)));
        assert!(points.contains(&Vector::new(100.0, 225.0)));
        assert!(points.contains(&Vector::new(150.0, 100.0)));
    }
    #[test]
    fn connect_points() {
        let grid = TiltedGrid::new(4, 4).tilt(0.25, -0.5);
        let points = grid.set_up_points(
            (400, 400),
            Vector::new(200.0, 200.0),
            consts::FRAC_PI_2,
            0.5,
        );
        let segments = grid.connect_points(&points);
        assert_eq!(segments.len(), 6);
        let segment = Segment::from(((300.0, 175.0), (100.0, 225.0)));
        assert!(segments.contains(&segment));
        let segment = Segment::from(((100.0, 112.5), (200.0, 312.5)));
        assert!(segments.contains(&segment));
    }
    #[test]
    fn intersect_segments() {
        let grid = TiltedGrid::new(4, 4).tilt(0.25, -0.5);
        let points = grid.set_up_points(
            (400, 400),
            Vector::new(200.0, 200.0),
            consts::FRAC_PI_2,
            0.5,
        );
        let segments = grid.connect_points(&points);
        let intersections = grid.intersect_segments(&segments);
        assert_eq!(intersections.len(), 9);
        assert!(intersections.contains(&Vector::new(200.0, 200.0)));
    }
}
