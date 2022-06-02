use std::f64::consts;

use palette::{Gradient, Mix};

use super::{ColoringMethod, Vector};

#[derive(Clone, Debug)]
pub struct ConicGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    gradient: Gradient<Color>,
    center_point: Vector,
    angle: f64,
    smoothness: f64,
}

impl<Color> ConicGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    pub fn new<ColorGradient>(
        gradient: ColorGradient,
        center_point: Vector,
        angle: f64,
        smoothness: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self {
            gradient: gradient.into(),
            center_point,
            angle: angle % (2.0 * consts::PI),
            smoothness: smoothness.clamp(0.0, 1.0),
        }
    }
    #[inline(always)]
    pub fn new_smooth<ColorGradient>(
        gradient: ColorGradient,
        center_point: Vector,
        angle: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new(gradient, center_point, angle, 1.0)
    }
    #[inline(always)]
    pub fn new_step<ColorGradient>(
        gradient: ColorGradient,
        center_point: Vector,
        angle: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new(gradient, center_point, angle, 0.0)
    }
    pub fn center_point(&self) -> Vector {
        self.center_point.clone()
    }
    pub fn set_center_point(&mut self, center_point: Vector) {
        self.center_point = center_point;
    }
    pub fn angle(&self) -> f64 {
        self.angle
    }
    pub fn set_angle(&mut self, angle: f64) {
        self.angle = angle % (2.0 * consts::PI);
    }
    pub fn smoothness(&self) -> f64 {
        self.smoothness
    }
    pub fn set_smoothness(&mut self, smoothness: f64) {
        self.smoothness = smoothness.clamp(0.0, 1.0);
    }
}

impl<Color> ColoringMethod<Color> for ConicGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    fn interpolate(&self, point: &Vector, center_point: &Vector) -> Color {
        let smoothed_point = center_point.interpolate(point, self.smoothness);
        let point_vector = &smoothed_point - &self.center_point;
        let angle = point_vector.y.atan2(point_vector.x) - self.angle;
        let clamped_angle = (angle + 2.0 * consts::PI) % (2.0 * consts::PI);
        self.gradient.get(clamped_angle / (2.0 * consts::PI))
    }
}

#[cfg(test)]
mod tests {
    use super::{super::tests, *};

    #[test]
    fn interpolate_smooth() {
        let gradient = tests::create_rgb_gradient();
        let conic_gradient = ConicGradient::new_smooth(
            gradient.clone(),
            Vector::new(100.0, 100.0),
            consts::FRAC_PI_4,
        );
        let center_point = Vector::new(100.0, 150.0);
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(150.0, 150.0), &center_point),
            gradient.get(0.0)
        );
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(100.0, 150.0), &center_point),
            gradient.get(0.125)
        );
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(50.0, 150.0), &center_point),
            gradient.get(0.25)
        );
        let center_point = Vector::new(100.0, 50.0);
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(50.0, 50.0), &center_point),
            gradient.get(0.5)
        );
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(100.0, 50.0), &center_point),
            gradient.get(0.625)
        );
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(150.0, 50.0), &center_point),
            gradient.get(0.75)
        );
    }
    #[test]
    fn interpolate_step() {
        let gradient = tests::create_lch_gradient();
        let conic_gradient = ConicGradient::new_step(
            gradient.clone(),
            Vector::new(100.0, 100.0),
            -consts::FRAC_PI_4,
        );
        let center_point = Vector::new(150.0, 150.0);
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(150.0, 150.0), &center_point),
            gradient.get(0.25)
        );
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(100.0, 150.0), &center_point),
            gradient.get(0.25)
        );
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(50.0, 150.0), &center_point),
            gradient.get(0.25)
        );
        let center_point = Vector::new(50.0, 50.0);
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(50.0, 50.0), &center_point),
            gradient.get(0.75)
        );
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(100.0, 50.0), &center_point),
            gradient.get(0.75)
        );
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(150.0, 50.0), &center_point),
            gradient.get(0.75)
        );
    }
    #[test]
    fn interpolate_semi_step() {
        let gradient = tests::create_hsl_gradient();
        let conic_gradient = ConicGradient::new(
            gradient.clone(),
            Vector::new(100.0, 100.0),
            consts::FRAC_PI_4,
            0.5,
        );
        let center_point = Vector::new(150.0, 100.0);
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(100.0, 150.0), &center_point),
            gradient.get(0.0)
        );
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(100.0, 50.0), &center_point),
            gradient.get(0.75)
        );
        let center_point = Vector::new(50.0, 100.0);
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(100.0, 150.0), &center_point),
            gradient.get(0.25)
        );
        assert_eq!(
            conic_gradient.interpolate(&Vector::new(100.0, 50.0), &center_point),
            gradient.get(0.5)
        );
    }
    #[test]
    fn interpolate_at_center() {
        let gradient = tests::create_lch_gradient();
        let conic_gradient =
            ConicGradient::new_smooth(gradient.clone(), Vector::new(100.0, 100.0), 0.0);
        assert_eq!(
            conic_gradient.interpolate(
                &conic_gradient.center_point(),
                &conic_gradient.center_point()
            ),
            gradient.get(0.0)
        );
    }
}
