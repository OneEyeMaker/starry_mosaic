use palette::Mix;

use super::vector::Vector;

pub trait ColoringMethod<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    fn interpolate(&self, point: &Vector, center_point: &Vector) -> Color;
}

mod linear_gradient;
mod radial_gradient;

pub use self::linear_gradient::LinearGradient;
pub use self::radial_gradient::RadialGradient;
