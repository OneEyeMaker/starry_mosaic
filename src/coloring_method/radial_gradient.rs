use palette::{Gradient, Mix};

use super::{super::utility, ColoringMethod, Vector};

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
    pub fn new<ColorGradient>(
        gradient: ColorGradient,
        inner_center: Vector,
        inner_radius: f64,
        outer_center: Vector,
        outer_radius: f64,
        smoothness: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        let direction = &outer_center - &inner_center;
        let direction_squared_length = direction.squared_length();
        let mut radial_gradient = Self {
            gradient: gradient.into(),
            inner_center,
            direction,
            direction_squared_length,
            inner_radius,
            radius_difference: outer_radius - inner_radius,
            smoothness: smoothness.clamp(0.0, 1.0),
        };
        radial_gradient.fit_inner_circle_into_outer();
        radial_gradient
    }
    #[inline(always)]
    pub fn new_smooth<ColorGradient>(
        gradient: ColorGradient,
        inner_center: Vector,
        inner_radius: f64,
        outer_center: Vector,
        outer_radius: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new(
            gradient,
            inner_center,
            inner_radius,
            outer_center,
            outer_radius,
            1.0,
        )
    }
    #[inline(always)]
    pub fn new_step<ColorGradient>(
        gradient: ColorGradient,
        inner_center: Vector,
        inner_radius: f64,
        outer_center: Vector,
        outer_radius: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new(
            gradient,
            inner_center,
            inner_radius,
            outer_center,
            outer_radius,
            0.0,
        )
    }
    #[inline(always)]
    pub fn new_simple<ColorGradient>(
        gradient: ColorGradient,
        center: Vector,
        radius: f64,
        smoothness: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new(gradient, center.clone(), 0.0, center, radius, smoothness)
    }
    #[inline(always)]
    pub fn new_simple_smooth<ColorGradient>(
        gradient: ColorGradient,
        center: Vector,
        radius: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new_simple(gradient, center, radius, 1.0)
    }
    #[inline(always)]
    pub fn new_simple_step<ColorGradient>(
        gradient: ColorGradient,
        center: Vector,
        radius: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new_simple(gradient, center, radius, 0.0)
    }
    pub fn inner_center(&self) -> Vector {
        self.inner_center.clone()
    }
    pub fn set_inner_center(&mut self, inner_center: Vector) {
        let outer_center = &self.inner_center + &self.direction;
        self.inner_center = inner_center;
        self.set_outer_center(outer_center);
    }
    pub fn inner_radius(&self) -> f64 {
        self.inner_radius
    }
    pub fn set_inner_radius(&mut self, inner_radius: f64) {
        let outer_radius = self.inner_radius + self.radius_difference;
        self.inner_radius = inner_radius;
        self.set_outer_radius(outer_radius);
    }
    pub fn outer_center(&self) -> Vector {
        &self.inner_center + &self.direction
    }
    pub fn set_outer_center(&mut self, outer_center: Vector) {
        self.direction = &outer_center - &self.inner_center;
        self.direction_squared_length = self.direction.squared_length();
        self.fit_inner_circle_into_outer();
    }
    pub fn outer_radius(&self) -> f64 {
        self.inner_radius + self.radius_difference
    }
    pub fn set_outer_radius(&mut self, outer_radius: f64) {
        self.radius_difference = outer_radius - self.inner_radius;
        self.fit_inner_circle_into_outer();
    }
    pub fn smoothness(&self) -> f64 {
        self.smoothness
    }
    pub fn set_smoothness(&mut self, smoothness: f64) {
        self.smoothness = smoothness.clamp(0.0, 1.0);
    }
    #[inline(always)]
    fn fit_inner_circle_into_outer(&mut self) {
        self.radius_difference = self
            .radius_difference
            .max(self.direction.length() + utility::EPSILON * 2.0);
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

#[cfg(test)]
mod tests {
    use super::{super::tests, *};

    #[test]
    fn set_inner_center() {
        let gradient = tests::create_rgb_gradient();
        let mut radial_gradient = RadialGradient::new_smooth(
            gradient,
            Vector::new(150.0, 250.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        radial_gradient.set_inner_center(Vector::new(50.0, 250.0));
        assert!(radial_gradient.radius_difference > radial_gradient.direction.length());
        assert!(radial_gradient.outer_radius() > 200.0);
    }
    #[test]
    fn set_inner_radius() {
        let gradient = tests::create_rgb_gradient();
        let mut radial_gradient = RadialGradient::new_smooth(
            gradient,
            Vector::new(150.0, 250.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        radial_gradient.set_inner_radius(150.0);
        assert!(radial_gradient.radius_difference > radial_gradient.direction.length());
        assert!(radial_gradient.outer_radius() > 200.0);
    }
    #[test]
    fn set_outer_center() {
        let gradient = tests::create_rgb_gradient();
        let mut radial_gradient = RadialGradient::new_smooth(
            gradient,
            Vector::new(150.0, 250.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        radial_gradient.set_outer_center(Vector::new(350.0, 250.0));
        assert!(radial_gradient.radius_difference > radial_gradient.direction.length());
        assert!(radial_gradient.outer_radius() > 200.0);
    }
    #[test]
    fn set_outer_radius() {
        let gradient = tests::create_rgb_gradient();
        let mut radial_gradient = RadialGradient::new_smooth(
            gradient,
            Vector::new(150.0, 250.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        radial_gradient.set_outer_radius(100.0);
        assert!(radial_gradient.radius_difference > radial_gradient.direction.length());
        assert!(radial_gradient.outer_radius() > 100.0);
    }
    #[test]
    fn interpolate_smooth() {
        let gradient = tests::create_rgb_gradient();
        let radial_gradient = RadialGradient::new_smooth(
            gradient.clone(),
            Vector::new(250.0, 150.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        let center_point = Vector::new(250.0, 325.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(250.0, 200.0 + index * 50.0);
            assert_eq!(
                radial_gradient.interpolate(&point, &center_point),
                gradient.get(index / 5.0)
            );
        }
        let center_point = Vector::new(250.0, 75.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(250.0, 100.0 - index * 10.0);
            assert_eq!(
                radial_gradient.interpolate(&point, &center_point),
                gradient.get(index / 5.0)
            );
        }
    }
    #[test]
    fn interpolate_step() {
        let gradient = tests::create_lch_gradient();
        let radial_gradient = RadialGradient::new_step(
            gradient.clone(),
            Vector::new(250.0, 150.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        let center_point = Vector::new(250.0, 325.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(250.0, 200.0 + index * 50.0);
            assert_eq!(
                radial_gradient.interpolate(&point, &center_point),
                gradient.get(0.5)
            );
        }
        let center_point = Vector::new(250.0, 75.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(250.0, 100.0 - index * 10.0);
            assert_eq!(
                radial_gradient.interpolate(&point, &center_point),
                gradient.get(0.5)
            );
        }
    }
    #[test]
    fn interpolate_semi_step() {
        let gradient = tests::create_hsl_gradient();
        let radial_gradient = RadialGradient::new(
            gradient.clone(),
            Vector::new(250.0, 150.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
            0.5,
        );
        let center_point = Vector::new(250.0, 325.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(250.0, 200.0 + index * 50.0);
            assert_eq!(
                radial_gradient.interpolate(&point, &center_point),
                gradient.get(0.25 + index / 10.0)
            );
        }
        let center_point = Vector::new(250.0, 75.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(250.0, 100.0 - index * 10.0);
            assert_eq!(
                radial_gradient.interpolate(&point, &center_point),
                gradient.get(0.25 + index / 10.0)
            );
        }
    }
    #[test]
    fn interpolate_center_position() {
        let gradient = tests::create_hsl_gradient();
        let radial_gradient = RadialGradient::new_smooth(
            gradient.clone(),
            Vector::new(150.0, 250.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        assert_eq!(
            radial_gradient
                .interpolate(&radial_gradient.inner_center, &radial_gradient.inner_center),
            gradient.get(0.0)
        );
    }
    #[test]
    fn interpolate_edge_positions() {
        let gradient = tests::create_lch_gradient();
        let radial_gradient = RadialGradient::new_smooth(
            gradient.clone(),
            Vector::new(350.0, 250.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        let point = Vector::new(0.0, 250.0);
        assert_eq!(
            radial_gradient.interpolate(&point, &radial_gradient.inner_center),
            gradient.get(1.0)
        );
        let point = Vector::new(500.0, 250.0);
        assert_eq!(
            radial_gradient.interpolate(&point, &radial_gradient.inner_center),
            gradient.get(1.0)
        );
        let point = Vector::new(250.0, 0.0);
        assert_eq!(
            radial_gradient.interpolate(&point, &radial_gradient.inner_center),
            gradient.get(1.0)
        );
        let point = Vector::new(250.0, 500.0);
        assert_eq!(
            radial_gradient.interpolate(&point, &radial_gradient.inner_center),
            gradient.get(1.0)
        );
    }
}
