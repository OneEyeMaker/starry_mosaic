use image::RgbImage;
use palette::{IntoColor, LinSrgb, Mix, Shade};

use super::{
    coloring_method::*,
    mosaic_shape::MosaicShape,
    transform::{Transformation, TryToTransform},
    vector::Vector,
};

/// Represents mosaic and allows to create mosaic images painted with different
/// [methods][`ColoringMethod`].
///
/// # Examples
///
/// This example creates mosaic that visualises its key points
/// (key points of [mosaic shape][`MosaicShape`]).
///
/// Uncomment lines at the end of `main` function to create blue mosaic image and save it to file.
///
/// ```
/// use image::{Rgb, RgbImage};
/// use palette::{IntoColor, LinSrgb, Mix, Pixel, Shade};
/// use starry_mosaic::{
///     coloring_method::ColoringMethod,
///     mosaic_shape::MosaicShape,
///     transform::{Scale, Transformation, TryToTransform},
///     Mosaic,
///     MosaicBuilder,
///     Vector
/// };
///
/// #[derive(Clone, Debug)]
/// struct DottedMosaic {
///     key_points: Vec<Vector>,
///     image_size: (u32, u32),
///     transformation: Transformation,
///     shape: Box<dyn MosaicShape>,
///     dot_radius: i32,
/// }
/// impl DottedMosaic {
///     fn new(
///         key_points: Vec<Vector>,
///         image_size: (u32, u32),
///         transformation: Transformation,
///         shape: Box<dyn MosaicShape>,
///     ) -> Self {
///         Self {
///             key_points,
///             image_size,
///             transformation,
///             shape,
///             dot_radius: 5,
///         }
///     }
///     fn dot_radius(&self) -> i32 {
///         self.dot_radius
///     }
///     fn set_dot_radius(&mut self, dot_radius: i32) {
///         self.dot_radius = dot_radius.max(1);
///     }
///     fn draw_dot<Color, Method>(
///         &self,
///         key_point: Vector,
///         coloring_method: &Method,
///         mosaic_image: &mut RgbImage
///     )
///     where
///         Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
///         Method: ColoringMethod<Color>,
///     {
///         let (image_width, image_height) = (self.image_size.0 as f64, self.image_size.1 as f64);
///         for x_shift in -self.dot_radius..=self.dot_radius {
///             for y_shift in -self.dot_radius..=self.dot_radius {
///                 let point = Vector::new(
///                     key_point.x + x_shift as f64,
///                     key_point.y + y_shift as f64
///                 );
///                 if point.x < 0.0
///                     || point.x >= image_width
///                     || point.y < 0.0
///                     || point.y >= image_height {
///                     continue;
///                 }
///                 if point.distance_to(key_point) > self.dot_radius as f64 {
///                     continue;
///                 }
///                 let color = coloring_method.interpolate(point, key_point).into_color();
///                 mosaic_image.put_pixel(
///                     point.x as u32,
///                     point.y as u32,
///                     Rgb(color.into_format().into_raw())
///                 );
///             }
///         }
///     }
/// }
/// impl Mosaic for DottedMosaic {
///     fn draw<Color, Method>(&self, coloring_method: Method) -> RgbImage
///     where
///         Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
///         Method: ColoringMethod<Color>,
///     {
///         let mut mosaic_image = RgbImage::new(self.image_size.0, self.image_size.1);
///         for key_point in &self.key_points {
///             self.draw_dot(*key_point, &coloring_method, &mut mosaic_image);
///         }
///         mosaic_image
///     }
///     fn image_size(&self) -> (u32, u32) {
///         self.image_size
///     }
///     fn transformation(&self) -> &Transformation {
///         &self.transformation
///     }
///     fn shape(&self) -> &Box<dyn MosaicShape> {
///         &self.shape
///     }
/// }
/// impl TryToTransform for DottedMosaic {
///     fn try_to_transform(&self, transformation: &Transformation) -> Option<Self> {
///         Some(MosaicBuilder::from(self)
///             .set_transformation(transformation)
///             .build_from_key_points(DottedMosaic::new))
///     }
/// }
///
/// fn main() {
///     let dotted_mosaic = MosaicBuilder::default()
///         .set_image_size(1024, 1024)
///         .set_center(Vector::new(512.0, 512.0))
///         .set_rotation_angle(45.0f64.to_radians())
///         .set_uniform_scale(0.75)
///         .build_from_key_points(DottedMosaic::new);
///
///     assert_eq!(dotted_mosaic.image_size(), (1024, 1024));
///     assert_eq!(dotted_mosaic.transformation().translation, Vector::new(512.0, 512.0));
///     assert_eq!(dotted_mosaic.transformation().rotation_angle, 45.0f64.to_radians());
///     assert_eq!(dotted_mosaic.transformation().scale, Scale::new_uniform(0.75));
///
///     // let blue_mosaic_image = dotted_mosaic.draw(LinSrgb::new(0.0f64, 0.0, 1.0));
///     // let save_result = blue_mosaic_image.save("target/dotted_mosaic.png");
///     // assert!(save_result.is_ok());
/// }
/// ```
pub trait Mosaic: TryToTransform {
    /// Creates mosaic image painted with specified coloring method.
    ///
    /// This method transforms abstract [mosaic shape][`MosaicShape`] (with its key points)
    /// to concrete pixels using given coloring method.
    ///
    /// # Arguments
    ///
    /// * `coloring_method`: [coloring method][`ColoringMethod`] used to draw every pixel
    /// of mosaic shape in image.
    ///
    /// returns: `RgbImage` - painted mosaic image containing mosaic shape (pattern).
    ///
    /// # See also
    ///
    /// * [`Mosaic`].
    ///
    fn draw<Color, Method>(&self, coloring_method: Method) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        Method: ColoringMethod<Color>;

    /// Width and height of mosaic and mosaic image it creates.
    fn image_size(&self) -> (u32, u32);

    /// Transformation (position, rotation, scale and shear) of [mosaic shape][`Mosaic::shape`]
    /// in mosaic.
    fn transformation(&self) -> &Transformation;

    /// Position of center of [mosaic shape][`Mosaic::shape`] in mosaic.
    fn center(&self) -> Vector {
        self.transformation().translation
    }

    /// Shape (pattern) of mosaic.
    fn shape(&self) -> &Box<dyn MosaicShape>;
}

#[cfg(feature = "mosaic_with_preset_coloring")]
use palette::Gradient;

/// Provides preset methods to create painted mosaic images.
///
/// This trait is implemented automatically for every implementer of `Mosaic` trait.
#[cfg(feature = "mosaic_with_preset_coloring")]
pub trait MosaicWithPresetColoring: Mosaic {
    /// Paints mosaic image using single color.
    ///
    /// # See also
    ///
    /// * [`Mosaic::draw`].
    ///
    fn draw_single_colored<Color>(&self, color: Color) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
    {
        self.draw(color)
    }

    /// Paints mosaic image using linear gradient.
    ///
    /// # See also
    ///
    /// * [`Mosaic::draw`].
    /// * [`LinearGradient::new`].
    ///
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

    /// Paints mosaic image using linear smooth gradient.
    ///
    /// # See also
    ///
    /// * [`Mosaic::draw`].
    /// * [`LinearGradient::new_smooth`].
    ///
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

    /// Paints mosaic image using linear step gradient.
    ///
    /// # See also
    ///
    /// * [`Mosaic::draw`].
    /// * [`LinearGradient::new_step`].
    ///
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

    /// Paints mosaic image using radial gradient.
    ///
    /// # See also
    ///
    /// * [`Mosaic::draw`].
    /// * [`RadialGradient::new`].
    ///
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

    /// Paints mosaic image using radial smooth gradient.
    ///
    /// # See also
    ///
    /// * [`Mosaic::draw`].
    /// * [`RadialGradient::new_smooth`].
    ///
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

    /// Paints mosaic image using radial step gradient.
    ///
    /// # See also
    ///
    /// * [`Mosaic::draw`].
    /// * [`RadialGradient::new_step`].
    ///
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

    /// Paints mosaic image using radial simple gradient.
    ///
    /// # See also
    ///
    /// * [`Mosaic::draw`].
    /// * [`RadialGradient::new_simple`].
    ///
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

    /// Paints mosaic image using radial simple smooth gradient.
    ///
    /// # See also
    ///
    /// * [`Mosaic::draw`].
    /// * [`RadialGradient::new_simple_smooth`].
    ///
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

    /// Paints mosaic image using radial simple step gradient.
    ///
    /// # See also
    ///
    /// * [`Mosaic::draw`].
    /// * [`RadialGradient::new_simple_step`].
    ///
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

    /// Paints mosaic image using conic gradient.
    ///
    /// # See also
    ///
    /// * [`Mosaic::draw`].
    /// * [`ConicGradient::new`].
    ///
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

    /// Paints mosaic image using conic smooth gradient.
    ///
    /// # See also
    ///
    /// * [`Mosaic::draw`].
    /// * [`ConicGradient::new_smooth`].
    ///
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

    /// Paints mosaic image using conic step gradient.
    ///
    /// # See also
    ///
    /// * [`Mosaic::draw`].
    /// * [`ConicGradient::new_step`].
    ///
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

#[cfg(feature = "mosaic_with_preset_coloring")]
impl<MosaicImage> MosaicWithPresetColoring for MosaicImage where MosaicImage: Mosaic {}
