//! This module provides various coloring methods to paint mosaic and so create painted
//! mosaic images.
//!
//! Primary method of coloring - is just a single color. So many color types provided by `palette`
//! crate are supported as coloring method of mosaic.
//!
//! Additionally this module provides various gradients to paint mosaic images. These gradients
//! can follow shape of mosaic (fully or partially) or ignore it completely.

use palette::Mix;

use super::vector::Vector;

/// Defines color of every pixel of every piece of mosaic image.
///
/// `ColoringMethod` generates color depending on position of pixel that is currently being drawn
/// and position of the key point of mosaic fragment.
///
/// # Examples
///
/// Next implementation paints every mosaic fragment in checkerboard pattern.
///
/// ```
/// use palette::{LinSrgb, Mix};
/// use starry_mosaic::{coloring_method::ColoringMethod, Vector};
///
/// struct CheckerboardPattern<Color>
/// where
///     Color: Mix<Scalar = f64> + Clone,
/// {
///     primary: Color,
///     secondary: Color,
/// }
/// impl<Color> ColoringMethod<Color> for CheckerboardPattern<Color>
/// where
///     Color: Mix<Scalar = f64> + Clone,
/// {
///     fn interpolate(&self, point: &Vector, key_point: &Vector) -> Color {
///         if (point.x < key_point.x) == (point.y < key_point.y) {
///             self.primary.clone()
///         } else {
///             self.secondary.clone()
///         }
///     }
/// }
///
/// fn main() {
///     let pattern = CheckerboardPattern {
///         primary: LinSrgb::new(1.0f64, 1.0, 0.0),
///         secondary: LinSrgb::new(0.0f64, 0.0, 1.0),
///     };
///     let key_point = Vector::new(100.0, 100.0);
///     assert_eq!(
///         pattern.interpolate(&Vector::new(50.0, 50.0), &key_point),
///         pattern.primary,
///     );
///     assert_eq!(
///         pattern.interpolate(&Vector::new(50.0, 150.0), &key_point),
///         pattern.secondary,
///     );
///     assert_eq!(
///         pattern.interpolate(&Vector::new(150.0, 50.0), &key_point),
///         pattern.secondary,
///     );
///     assert_eq!(
///         pattern.interpolate(&Vector::new(150.0, 150.0), &key_point),
///         pattern.primary,
///     );
/// }
/// ```
pub trait ColoringMethod<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    /// Defines color of current pixel by interpolating between its position and
    /// position of the key point of mosaic fragment.
    ///
    /// # Arguments
    ///
    /// * `point`: position of pixel that is currently being drawn.
    /// * `key_point`: position of key point of current mosaic fragment.
    ///
    /// returns: `Color` - color of current pixel of mosaic image.
    ///
    /// # See also
    ///
    /// * [`ColoringMethod`].
    ///
    fn interpolate(&self, point: &Vector, key_point: &Vector) -> Color;
}

impl<Color> ColoringMethod<Color> for Color
where
    Color: Mix<Scalar = f64> + Clone,
{
    #[inline(always)]
    fn interpolate(&self, _point: &Vector, _key_point: &Vector) -> Color {
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
