use image::RgbImage;
use palette::{IntoColor, LinSrgb, Mix, Shade};

use super::coloring_method::*;

pub trait Mosaic {
    fn draw<Color, Method>(&self, coloring_method: Method) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        Method: ColoringMethod<Color>;
}

#[cfg(feature = "mosaic_with_preset_coloring")]
use palette::Gradient;

#[cfg(feature = "mosaic_with_preset_coloring")]
use super::vector::Vector;

#[cfg(feature = "mosaic_with_preset_coloring")]
pub trait MosaicWithPresetColoring: Mosaic {
    fn draw_single_colored<Color>(&self, color: Color) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
    {
        self.draw(color)
    }
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
    fn draw_linear_smooth_gradient<Color, ColorGradient>(
        &self,
        gradient: ColorGradient,
        start_point: Vector,
        end_point: Vector,
    ) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        ColorGradient: Into<Gradient<Color>>,
    {
        self.draw(LinearGradient::new_smooth(gradient, start_point, end_point))
    }
    fn draw_linear_step_gradient<Color, ColorGradient>(
        &self,
        gradient: ColorGradient,
        start_point: Vector,
        end_point: Vector,
    ) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        ColorGradient: Into<Gradient<Color>>,
    {
        self.draw(LinearGradient::new_step(gradient, start_point, end_point))
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
    fn draw_radial_smooth_gradient<Color, ColorGradient>(
        &self,
        gradient: ColorGradient,
        inner_center: Vector,
        inner_radius: f64,
        outer_center: Vector,
        outer_radius: f64,
    ) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        ColorGradient: Into<Gradient<Color>>,
    {
        self.draw(RadialGradient::new_smooth(
            gradient,
            inner_center,
            inner_radius,
            outer_center,
            outer_radius,
        ))
    }
    fn draw_radial_step_gradient<Color, ColorGradient>(
        &self,
        gradient: ColorGradient,
        inner_center: Vector,
        inner_radius: f64,
        outer_center: Vector,
        outer_radius: f64,
    ) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        ColorGradient: Into<Gradient<Color>>,
    {
        self.draw(RadialGradient::new_step(
            gradient,
            inner_center,
            inner_radius,
            outer_center,
            outer_radius,
        ))
    }
    fn draw_radial_simple_gradient<Color, ColorGradient>(
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
    fn draw_radial_simple_smooth_gradient<Color, ColorGradient>(
        &self,
        gradient: ColorGradient,
        center: Vector,
        radius: f64,
    ) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        ColorGradient: Into<Gradient<Color>>,
    {
        self.draw(RadialGradient::new_simple_smooth(gradient, center, radius))
    }
    fn draw_radial_simple_step_gradient<Color, ColorGradient>(
        &self,
        gradient: ColorGradient,
        center: Vector,
        radius: f64,
    ) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        ColorGradient: Into<Gradient<Color>>,
    {
        self.draw(RadialGradient::new_simple_step(gradient, center, radius))
    }
    fn draw_conic_gradient<Color, ColorGradient>(
        &self,
        gradient: ColorGradient,
        center: Vector,
        angle: f64,
        smoothness: f64,
    ) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        ColorGradient: Into<Gradient<Color>>,
    {
        self.draw(ConicGradient::new(gradient, center, angle, smoothness))
    }
    fn draw_conic_smooth_gradient<Color, ColorGradient>(
        &self,
        gradient: ColorGradient,
        center: Vector,
        angle: f64,
    ) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        ColorGradient: Into<Gradient<Color>>,
    {
        self.draw(ConicGradient::new_smooth(gradient, center, angle))
    }
    fn draw_conic_step_gradient<Color, ColorGradient>(
        &self,
        gradient: ColorGradient,
        center: Vector,
        angle: f64,
    ) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        ColorGradient: Into<Gradient<Color>>,
    {
        self.draw(ConicGradient::new_step(gradient, center, angle))
    }
}
