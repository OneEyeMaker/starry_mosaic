use super::{MosaicShape, Segment, Vector};

/// Defines mosaic shape based on grid.
#[derive(Clone, Debug)]
pub struct Grid {
    rows_count: u32,
    columns_count: u32,
}

impl Grid {
    /// Creates grid with set number of rows and columns.
    ///
    /// # Arguments
    ///
    /// * `rows_count`: number of grid rows; should be at least 1.
    /// * `columns_count`: number of grid columns; should be at least 1.
    ///
    /// returns: [`Grid`] - mosaic shape based on grid with given number of rows and columns.
    ///
    pub fn new(rows_count: u32, columns_count: u32) -> Self {
        Self {
            rows_count: rows_count.max(1),
            columns_count: columns_count.max(1),
        }
    }

    /// Number of rows of grid on which mosaic shape is based.
    #[inline(always)]
    pub fn rows_count(&self) -> u32 {
        self.rows_count
    }

    /// Number of columns of grid on which mosaic shape is based.
    #[inline(always)]
    pub fn columns_count(&self) -> u32 {
        self.columns_count
    }

    /// Sets number of rows of grid on which mosaic shape is based.
    ///
    /// # Arguments
    ///
    /// * `rows_count`: number of grid rows; should be at least 1.
    ///
    pub fn set_rows_count(&mut self, rows_count: u32) {
        self.rows_count = rows_count.max(1);
    }

    /// Sets number of columns of grid on which mosaic shape is based.
    ///
    /// # Arguments
    ///
    /// * `columns_count`: number of grid columns; should be at least 1.
    ///
    pub fn set_columns_count(&mut self, columns_count: u32) {
        self.columns_count = columns_count.max(1);
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            rows_count: 4,
            columns_count: 4,
        }
    }
}

impl MosaicShape for Grid {
    fn set_up_points(&self, image_width: u32, image_height: u32) -> Vec<Vector> {
        let (image_width, image_height) = (image_width as f64, image_height as f64);
        let (horizontal_step_size, vertical_step_size) = (
            image_width / self.columns_count as f64,
            image_height / self.rows_count as f64,
        );
        let step_size = horizontal_step_size.min(vertical_step_size);
        let (horizontal_half_size, vertical_half_size) = (
            step_size * self.columns_count as f64 * 0.5,
            step_size * self.rows_count as f64 * 0.5,
        );
        let mut points = vec![];
        points.push(Vector::new(-horizontal_half_size, -vertical_half_size));
        points.push(Vector::new(-horizontal_half_size, vertical_half_size));
        points.push(Vector::new(horizontal_half_size, -vertical_half_size));
        points.push(Vector::new(horizontal_half_size, vertical_half_size));
        for index in 1..self.rows_count {
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
        for index in 1..self.columns_count {
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
    }

    fn connect_points(&self, shape_points: &Vec<Vector>) -> Vec<Segment> {
        let mut segments = vec![];
        let points_count = shape_points.len();
        for index in (4..points_count).step_by(2) {
            segments.push(Segment::new(shape_points[index], shape_points[index + 1]));
        }
        segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_rows_count() {
        let mut grid = Grid::default();
        grid.set_rows_count(7);
        assert_eq!(grid.rows_count(), 7);
    }
    #[test]
    fn set_incorrect_rows_count() {
        let mut grid = Grid::default();
        grid.set_rows_count(0);
        assert_eq!(grid.rows_count(), 1);
    }
    #[test]
    fn set_columns_count() {
        let mut grid = Grid::default();
        grid.set_columns_count(15);
        assert_eq!(grid.columns_count(), 15);
    }
    #[test]
    fn set_incorrect_columns_count() {
        let mut grid = Grid::default();
        grid.set_columns_count(0);
        assert_eq!(grid.columns_count(), 1);
    }
    #[test]
    fn set_up_points() {
        let grid = Grid::new(4, 4);
        let points = grid.set_up_points(400, 400);
        assert_eq!(points.len(), 16);
        assert!(points.contains(&Vector::new(200.0, -200.0)));
        assert!(points.contains(&Vector::new(100.0, 200.0)));
        assert!(points.contains(&Vector::new(-100.0, -200.0)));
        assert!(points.contains(&Vector::new(0.0, 200.0)));
    }
    #[test]
    fn connect_points() {
        let grid = Grid::new(4, 4);
        let points = grid.set_up_points(400, 400);
        let segments = grid.connect_points(&points);
        assert_eq!(segments.len(), 6);
        let segment = Segment::from(((-200.0, 100.0), (200.0, 100.0)));
        assert!(segments.contains(&segment));
        let segment = Segment::from(((100.0, -200.0), (100.0, 200.0)));
        assert!(segments.contains(&segment));
    }
    #[test]
    fn intersect_segments() {
        let grid = Grid::new(4, 4);
        let points = grid.set_up_points(400, 400);
        let segments = grid.connect_points(&points);
        let intersections = grid.intersect_segments(&segments);
        assert_eq!(intersections.len(), 9);
        assert!(intersections.contains(&Vector::new(100.0, 100.0)));
    }
}
