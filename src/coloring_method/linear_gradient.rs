use palette::{Gradient, Mix, Shade};

use super::{ColoringMethod, Vector};

#[derive(Clone, Debug)]
pub struct LinearGradient<Color>
    where
        Color: Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
{
    gradient: Gradient<Color>,
    start_point: Vector,
    direction: Vector,
    direction_squared_length: f64,
    smoothness: f64,
}

impl<Color> LinearGradient<Color>
    where
        Color: Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
{
    pub fn new(
        colors: &[(f64, Color)],
        start_point: Vector,
        end_point: Vector,
        smoothness: f64,
    ) -> Self {
        let gradient = Gradient::with_domain(Vec::from(colors));
        let direction = if start_point != end_point {
            &end_point - &start_point
        } else {
            Vector::new(1.0, 0.0)
        };
        let direction_squared_length = direction.squared_length();
        Self {
            gradient,
            start_point,
            direction,
            direction_squared_length,
            smoothness: smoothness.clamp(0.0, 1.0),
        }
    }
    #[inline(always)]
    pub fn new_smooth(colors: &[(f64, Color)], start_point: Vector, end_point: Vector) -> Self {
        Self::new(colors, start_point, end_point, 1.0)
    }
    #[inline(always)]
    pub fn new_step(colors: &[(f64, Color)], start_point: Vector, end_point: Vector) -> Self {
        Self::new(colors, start_point, end_point, 0.0)
    }
}

impl<Color> ColoringMethod<Color> for LinearGradient<Color>
    where
        Color: Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
{
    fn interpolate(&self, point: &Vector, center_point: &Vector, distance_limit: f64) -> Color {
        let smoothed_point = center_point.interpolate(point, self.smoothness);
        let interpolation_factor = (&smoothed_point - &self.start_point).dot(&self.direction)
            / self.direction_squared_length;
        let distance = point.distance_to(center_point);
        let lighten_factor = (1.0 - distance / distance_limit).powi(2);
        self.gradient
            .get(interpolation_factor)
            .lighten(lighten_factor)
    }
}