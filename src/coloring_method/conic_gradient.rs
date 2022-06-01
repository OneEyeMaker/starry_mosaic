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
    pub fn new(colors: &[(f64, Color)], center_point: Vector, angle: f64, smoothness: f64) -> Self {
        let gradient = Gradient::with_domain(Vec::from(colors));
        Self {
            gradient,
            center_point,
            angle: angle % (2.0 * consts::PI),
            smoothness: smoothness.clamp(0.0, 1.0),
        }
    }
    #[inline(always)]
    pub fn new_smooth(colors: &[(f64, Color)], center_point: Vector, angle: f64) -> Self {
        Self::new(colors, center_point, angle, 1.0)
    }
    #[inline(always)]
    pub fn new_step(colors: &[(f64, Color)], center_point: Vector, angle: f64) -> Self {
        Self::new(colors, center_point, angle, 0.0)
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
