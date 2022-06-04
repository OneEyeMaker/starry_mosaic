use palette::Mix;

use super::vector::Vector;

pub trait ColoringMethod<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    fn interpolate(&self, point: &Vector, center_point: &Vector) -> Color;
}

impl<Color> ColoringMethod<Color> for Color
where
    Color: Mix<Scalar = f64> + Clone,
{
    #[inline(always)]
    fn interpolate(&self, _point: &Vector, _center_point: &Vector) -> Color {
        self.clone()
    }
}

mod conic_gradient;
mod linear_gradient;
mod radial_gradient;

pub use self::conic_gradient::ConicGradient;
pub use self::linear_gradient::LinearGradient;
pub use self::radial_gradient::RadialGradient;

#[cfg(test)]
mod tests {
    use palette::{encoding::Srgb, white_point::D65, Gradient, Hsl, Lch, LinSrgb};

    pub fn create_rgb_gradient() -> Gradient<LinSrgb<f64>> {
        Gradient::from(vec![
            (0.1, LinSrgb::new(1.0, 0.0, 0.0)),
            (0.5, LinSrgb::new(0.0, 1.0, 0.0)),
            (0.7, LinSrgb::new(0.0, 0.0, 1.0)),
        ])
    }
    pub fn create_hsl_gradient() -> Gradient<Hsl<Srgb, f64>> {
        Gradient::from(vec![
            (0.3, Hsl::new(0.0, 1.0, 0.5)),
            (0.75, Hsl::new(120.0, 1.0, 0.5)),
            (1.0, Hsl::new(240.0, 1.0, 0.5)),
        ])
    }
    pub fn create_lch_gradient() -> Gradient<Lch<D65, f64>> {
        Gradient::from(vec![
            (0.0, Lch::new(50.0, 100.0, 40.0)),
            (0.25, Lch::new(90.0, 110.0, 130.0)),
            (0.6, Lch::new(30.0, 130.0, 300.0)),
        ])
    }
}
