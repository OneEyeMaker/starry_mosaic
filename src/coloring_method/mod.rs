use palette::{Mix, Shade};

use super::vector::Vector;

pub trait ColoringMethod<Color>
where
    Color: Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
{
    fn interpolate(&self, point: &Vector, center_point: &Vector, distance_limit: f64) -> Color;
}

impl<Color> ColoringMethod<Color> for Color
where
    Color: Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
{
    fn interpolate(&self, point: &Vector, center_point: &Vector, distance_limit: f64) -> Color {
        let distance = point.distance_to(center_point);
        let lighten_factor = (1.0 - distance / distance_limit).powi(2);
        self.lighten(lighten_factor)
    }
}

mod linear_gradient;

pub use self::linear_gradient::LinearGradient;
