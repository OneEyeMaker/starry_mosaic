use image::RgbImage;
use palette::{Gradient, IntoColor, LinSrgb, Mix, Shade};

use super::{coloring_method::*, vector::Vector};

pub trait Mosaic {
    fn draw_linear_gradient<Color, ColorGradient>(
        &self,
        gradient: ColorGradient,
        start_point: Vector,
        end_point: Vector,
        smoothness: f64,
    ) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        ColorGradient: Into<Gradient<Color>>,
    {
        self.draw(LinearGradient::new(
            gradient,
            start_point,
            end_point,
            smoothness,
        ))
    }
    fn draw_radial_gradient<Color, ColorGradient>(
        &self,
        gradient: ColorGradient,
        inner_center: Vector,
        inner_radius: f64,
        outer_center: Vector,
        outer_radius: f64,
        smoothness: f64,
    ) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        ColorGradient: Into<Gradient<Color>>,
    {
        self.draw(RadialGradient::new(
            gradient,
            inner_center,
            inner_radius,
            outer_center,
            outer_radius,
            smoothness,
        ))
    }
    fn draw_simple_radial_gradient<Color, ColorGradient>(
        &self,
        gradient: ColorGradient,
        center: Vector,
        radius: f64,
        smoothness: f64,
    ) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        ColorGradient: Into<Gradient<Color>>,
    {
        self.draw(RadialGradient::new_simple(
            gradient, center, radius, smoothness,
        ))
    }
    fn draw_conic_gradient<Color, ColorGradient>(
        &self,
        gradient: ColorGradient,
        center_point: Vector,
        angle: f64,
        smoothness: f64,
    ) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        ColorGradient: Into<Gradient<Color>>,
    {
        self.draw(ConicGradient::new(
            gradient,
            center_point,
            angle,
            smoothness,
        ))
    }
    fn draw<Color, Method>(&self, coloring_method: Method) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        Method: ColoringMethod<Color>;
}
