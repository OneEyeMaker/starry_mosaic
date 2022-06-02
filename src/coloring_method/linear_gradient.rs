use palette::{Gradient, Mix};

use super::{ColoringMethod, Vector};

#[derive(Clone, Debug)]
pub struct LinearGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    gradient: Gradient<Color>,
    start_point: Vector,
    direction: Vector,
    direction_squared_length: f64,
    smoothness: f64,
}

impl<Color> LinearGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    pub fn new<ColorGradient>(
        gradient: ColorGradient,
        start_point: Vector,
        end_point: Vector,
        smoothness: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        let direction = if start_point != end_point {
            &end_point - &start_point
        } else {
            Vector::new(1.0, 0.0)
        };
        let direction_squared_length = direction.squared_length();
        Self {
            gradient: gradient.into(),
            start_point,
            direction,
            direction_squared_length,
            smoothness: smoothness.clamp(0.0, 1.0),
        }
    }
    #[inline(always)]
    pub fn new_smooth<ColorGradient>(
        gradient: ColorGradient,
        start_point: Vector,
        end_point: Vector,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new(gradient, start_point, end_point, 1.0)
    }
    #[inline(always)]
    pub fn new_step<ColorGradient>(
        gradient: ColorGradient,
        start_point: Vector,
        end_point: Vector,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new(gradient, start_point, end_point, 0.0)
    }
    pub fn start_point(&self) -> Vector {
        self.start_point.clone()
    }
    pub fn set_start_point(&mut self, start_point: Vector) {
        let end_point = &self.start_point + &self.direction;
        self.start_point = start_point;
        self.set_direction(end_point);
    }
    pub fn end_point(&self) -> Vector {
        &self.start_point + &self.direction
    }
    pub fn set_end_point(&mut self, end_point: Vector) {
        self.set_direction(end_point);
    }
    pub fn smoothness(&self) -> f64 {
        self.smoothness
    }
    pub fn set_smoothness(&mut self, smoothness: f64) {
        self.smoothness = smoothness.clamp(0.0, 1.0);
    }
    #[inline(always)]
    fn set_direction(&mut self, end_point: Vector) {
        self.direction = if self.start_point != end_point {
            &end_point - &self.start_point
        } else {
            Vector::new(1.0, 0.0)
        };
        self.direction_squared_length = self.direction.squared_length();
    }
}

impl<Color> ColoringMethod<Color> for LinearGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    fn interpolate(&self, point: &Vector, center_point: &Vector) -> Color {
        let smoothed_point = center_point.interpolate(point, self.smoothness);
        let interpolation_factor = (&smoothed_point - &self.start_point).dot(&self.direction)
            / self.direction_squared_length;
        self.gradient.get(interpolation_factor)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::tests, *};

    #[test]
    fn create_with_null_direction_vector() {
        let gradient = tests::create_rgb_gradient();
        let point = Vector::new(100.0, 100.0);
        let linear_gradient = LinearGradient::new_smooth(gradient, point.clone(), point);
        assert!(linear_gradient.direction_squared_length > 0.0);
    }
    #[test]
    fn set_start_point_equal_to_end_point() {
        let gradient = tests::create_hsl_gradient();
        let mut linear_gradient =
            LinearGradient::new_smooth(gradient, Vector::new(0.0, 0.0), Vector::new(100.0, 100.0));
        linear_gradient.set_start_point(linear_gradient.end_point());
        assert!(linear_gradient.direction_squared_length > 0.0);
    }
    #[test]
    fn set_end_point_equal_to_start_point() {
        let gradient = tests::create_lch_gradient();
        let mut linear_gradient =
            LinearGradient::new_smooth(gradient, Vector::new(0.0, 0.0), Vector::new(100.0, 100.0));
        linear_gradient.set_end_point(linear_gradient.start_point());
        assert!(linear_gradient.direction_squared_length > 0.0);
    }
    #[test]
    fn interpolate_smooth() {
        let gradient = tests::create_rgb_gradient();
        let linear_gradient = LinearGradient::new_smooth(
            gradient.clone(),
            Vector::new(0.0, 0.0),
            Vector::new(100.0, 100.0),
        );
        let center_point = Vector::new(25.0, 25.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(index * 10.0, index * 10.0);
            assert_eq!(
                linear_gradient.interpolate(&point, &center_point),
                gradient.get(index / 10.0)
            );
        }
        let center_point = Vector::new(75.0, 75.0);
        for index in 5..=10 {
            let index = index as f64;
            let point = Vector::new(index * 10.0, index * 10.0);
            assert_eq!(
                linear_gradient.interpolate(&point, &center_point),
                gradient.get(index / 10.0)
            );
        }
    }
    #[test]
    fn interpolate_step() {
        let gradient = tests::create_lch_gradient();
        let linear_gradient = LinearGradient::new_step(
            gradient.clone(),
            Vector::new(0.0, 0.0),
            Vector::new(100.0, 100.0),
        );
        let center_point = Vector::new(25.0, 25.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(index * 10.0, index * 10.0);
            assert_eq!(
                linear_gradient.interpolate(&point, &center_point),
                gradient.get(0.25)
            );
        }
        let center_point = Vector::new(75.0, 75.0);
        for index in 5..=10 {
            let index = index as f64;
            let point = Vector::new(index * 10.0, index * 10.0);
            assert_eq!(
                linear_gradient.interpolate(&point, &center_point),
                gradient.get(0.75)
            );
        }
    }
    #[test]
    fn interpolate_semi_step() {
        let gradient = tests::create_hsl_gradient();
        let linear_gradient = LinearGradient::new(
            gradient.clone(),
            Vector::new(0.0, 0.0),
            Vector::new(100.0, 100.0),
            0.5,
        );
        let center_point = Vector::new(25.0, 25.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(index * 10.0, index * 10.0);
            assert_eq!(
                linear_gradient.interpolate(&point, &center_point),
                gradient.get(0.125 + index / 20.0)
            );
        }
        let center_point = Vector::new(75.0, 75.0);
        for index in 5..=10 {
            let index = index as f64;
            let point = Vector::new(index * 10.0, index * 10.0);
            assert_eq!(
                linear_gradient.interpolate(&point, &center_point),
                gradient.get(0.625 + (index - 5.0) / 20.0)
            );
        }
    }
    #[test]
    fn interpolate_with_minimal_distance() {
        let gradient = tests::create_rgb_gradient();
        let start_point = Vector::new(50.0, 50.0);
        let end_point = Vector::new(51.0, 50.0);
        let linear_gradient =
            LinearGradient::new_smooth(gradient.clone(), start_point.clone(), end_point.clone());
        assert_ne!(
            linear_gradient.interpolate(&start_point, &start_point),
            linear_gradient.interpolate(&end_point, &end_point)
        );
    }
}
