use palette::{Gradient, Mix};

use super::{utility, ColoringMethod, Vector};

#[derive(Clone, Debug)]
pub struct RadialGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    gradient: Gradient<Color>,
    inner_center: Vector,
    direction: Vector,
    direction_squared_length: f64,
    inner_radius: f64,
    radius_difference: f64,
    smoothness: f64,
}

impl<Color> RadialGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    pub fn new(
        colors: &[(f64, Color)],
        inner_center: Vector,
        inner_radius: f64,
        outer_center: Vector,
        outer_radius: f64,
        smoothness: f64,
    ) -> Self {
        let gradient = Gradient::with_domain(Vec::from(colors));
        let direction = &outer_center - &inner_center;
        let direction_squared_length = direction.squared_length();
        let radius_difference = if utility::approx_eq(inner_radius, outer_radius) {
            1.0
        } else {
            outer_radius - inner_radius
        };
        Self {
            gradient,
            inner_center,
            direction,
            direction_squared_length,
            inner_radius,
            radius_difference,
            smoothness: smoothness.clamp(0.0, 1.0),
        }
    }
    #[inline(always)]
    pub fn new_smooth(
        colors: &[(f64, Color)],
        inner_center: Vector,
        inner_radius: f64,
        outer_center: Vector,
        outer_radius: f64,
    ) -> Self {
        Self::new(
            colors,
            inner_center,
            inner_radius,
            outer_center,
            outer_radius,
            1.0,
        )
    }
    #[inline(always)]
    pub fn new_step(
        colors: &[(f64, Color)],
        inner_center: Vector,
        inner_radius: f64,
        outer_center: Vector,
        outer_radius: f64,
    ) -> Self {
        Self::new(
            colors,
            inner_center,
            inner_radius,
            outer_center,
            outer_radius,
            0.0,
        )
    }
    #[inline(always)]
    pub fn new_simple(
        colors: &[(f64, Color)],
        center: Vector,
        radius: f64,
        smoothness: f64,
    ) -> Self {
        Self::new(colors, center.clone(), 0.0, center, radius, smoothness)
    }
    #[inline(always)]
    pub fn new_simple_smooth(colors: &[(f64, Color)], center: Vector, radius: f64) -> Self {
        Self::new_simple(colors, center, radius, 1.0)
    }
    #[inline(always)]
    pub fn new_simple_step(colors: &[(f64, Color)], center: Vector, radius: f64) -> Self {
        Self::new_simple(colors, center, radius, 0.0)
    }
}

impl<Color> ColoringMethod<Color> for RadialGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    fn interpolate(&self, point: &Vector, center_point: &Vector) -> Color {
        let smoothed_point = center_point.interpolate(point, self.smoothness);
        let point_vector = &smoothed_point - &self.inner_center;
        let alpha = self.direction_squared_length - self.radius_difference.powi(2);
        let beta = point_vector.dot(&self.direction) + self.inner_radius * self.radius_difference;
        let gamma = point_vector.squared_length() - self.inner_radius.powi(2);
        let discriminant = beta * beta - alpha * gamma;
        let interpolation_factor = (beta - discriminant.sqrt()) / alpha;
        self.gradient.get(interpolation_factor)
    }
}
