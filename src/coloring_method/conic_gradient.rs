use std::f64::consts;

use palette::{Gradient, Mix};

use super::{ColoringMethod, Vector};

/// Defines conic gradient for painting mosaic images.
#[derive(Clone, Debug)]
pub struct ConicGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    gradient: Gradient<Color>,
    center: Vector,
    angle: f64,
    smoothness: f64,
}

impl<Color> ConicGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    /// Creates conic gradient around given point.
    ///
    /// # Arguments
    ///
    /// * `gradient`: list of colors or colors stops of gradient.
    /// * `center`: center point around which the gradient is drawn.
    /// * `angle`: angle at which to begin the gradient, in radians.
    /// * `smoothness`: smoothness of gradient ranging from 0.0 to 1.0;
    /// see [`ConicGradient::smoothness`] for more information.
    ///
    /// returns: ConicGradient<Color> - conic gradient around center point.
    ///
    /// # Examples
    ///
    /// Next example creates conic semi-step gradient around point with coordinates (100.0, 100.0).
    /// ```
    /// use std::f64::consts;
    ///
    /// use palette::LinSrgb;
    /// use starry_mosaic::{coloring_method::{ColoringMethod, ConicGradient}, Vector};
    ///
    /// let gradient = vec![
    ///     (0.0, LinSrgb::new(1.0f64, 0.0, 0.0)),
    ///     (0.25, LinSrgb::new(1.0f64, 1.0, 0.0)),
    ///     (0.5, LinSrgb::new(0.0f64, 0.0, 1.0)),
    ///     (0.75, LinSrgb::new(1.0f64, 1.0, 0.0)),
    ///     (1.0, LinSrgb::new(1.0f64, 0.0, 0.0)),
    /// ];
    /// let conic_semi_step_gradient = ConicGradient::new(
    ///     gradient,
    ///     Vector::new(100.0, 100.0),
    ///     -consts::FRAC_PI_2,
    ///     0.5,
    /// );
    ///
    /// let key_point = Vector::new(200.0, 100.0);
    /// assert_eq!(
    ///     conic_semi_step_gradient.interpolate(Vector::new(100.0, 0.0), key_point),
    ///     LinSrgb::new(1.0f64, 0.5, 0.0),
    /// );
    /// ```
    pub fn new<ColorGradient>(
        gradient: ColorGradient,
        center: Vector,
        angle: f64,
        smoothness: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self {
            gradient: gradient.into(),
            center,
            angle: angle % consts::TAU,
            smoothness: smoothness.clamp(0.0, 1.0),
        }
    }

    /// Creates conic smooth gradient around given point.
    ///
    /// # Arguments
    ///
    /// * `gradient`: list of colors or colors stops of gradient.
    /// * `center`: center point around which the gradient is drawn.
    /// * `angle`: angle at which to begin the gradient, in radians.
    ///
    /// returns: ConicGradient<Color> - conic smooth gradient around center point.
    ///
    /// # See also
    ///
    /// * [`ConicGradient::new`].
    /// * [`ConicGradient::smoothness`].
    ///
    /// # Examples
    ///
    /// Next example creates conic smooth gradient around point with coordinates (100.0, 100.0).
    /// ```
    /// use std::f64::consts;
    ///
    /// use palette::LinSrgb;
    /// use starry_mosaic::{coloring_method::{ColoringMethod, ConicGradient}, Vector};
    ///
    /// let gradient = vec![
    ///     (0.0, LinSrgb::new(1.0f64, 0.0, 0.0)),
    ///     (0.25, LinSrgb::new(1.0f64, 1.0, 0.0)),
    ///     (0.5, LinSrgb::new(0.0f64, 0.0, 1.0)),
    ///     (0.75, LinSrgb::new(1.0f64, 1.0, 0.0)),
    ///     (1.0, LinSrgb::new(1.0f64, 0.0, 0.0)),
    /// ];
    /// let conic_smooth_gradient = ConicGradient::new_smooth(
    ///     gradient,
    ///     Vector::new(100.0, 100.0),
    ///     -consts::FRAC_PI_2,
    /// );
    ///
    /// let key_point = Vector::new(200.0, 100.0);
    /// assert_eq!(
    ///     conic_smooth_gradient.interpolate(Vector::new(100.0, 0.0), key_point),
    ///     LinSrgb::new(1.0f64, 0.0, 0.0),
    /// );
    /// ```
    #[inline(always)]
    pub fn new_smooth<ColorGradient>(gradient: ColorGradient, center: Vector, angle: f64) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new(gradient, center, angle, 1.0)
    }

    /// Creates conic step gradient around given point.
    ///
    /// # Arguments
    ///
    /// * `gradient`: list of colors or colors stops of gradient.
    /// * `center`: center point around which the gradient is drawn.
    /// * `angle`: angle at which to begin the gradient, in radians.
    ///
    /// returns: ConicGradient<Color> - conic step gradient around center point.
    ///
    /// # See also
    ///
    /// * [`ConicGradient::new`].
    /// * [`ConicGradient::smoothness`].
    ///
    /// # Examples
    ///
    /// Next example creates conic step gradient around point with coordinates (100.0, 100.0).
    /// ```
    /// use std::f64::consts;
    ///
    /// use palette::LinSrgb;
    /// use starry_mosaic::{coloring_method::{ColoringMethod, ConicGradient}, Vector};
    ///
    /// let gradient = vec![
    ///     (0.0, LinSrgb::new(1.0f64, 0.0, 0.0)),
    ///     (0.25, LinSrgb::new(1.0f64, 1.0, 0.0)),
    ///     (0.5, LinSrgb::new(0.0f64, 0.0, 1.0)),
    ///     (0.75, LinSrgb::new(1.0f64, 1.0, 0.0)),
    ///     (1.0, LinSrgb::new(1.0f64, 0.0, 0.0)),
    /// ];
    /// let conic_step_gradient = ConicGradient::new_step(
    ///     gradient,
    ///     Vector::new(100.0, 100.0),
    ///     -consts::FRAC_PI_2,
    /// );
    ///
    /// let key_point = Vector::new(200.0, 100.0);
    /// assert_eq!(
    ///     conic_step_gradient.interpolate(Vector::new(100.0, 0.0), key_point),
    ///     LinSrgb::new(1.0f64, 1.0, 0.0),
    /// );
    /// ```
    #[inline(always)]
    pub fn new_step<ColorGradient>(gradient: ColorGradient, center: Vector, angle: f64) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new(gradient, center, angle, 0.0)
    }

    /// Center point around which conic gradient is drawn.
    pub fn center(&self) -> Vector {
        self.center
    }

    /// Sets center point around which conic gradient is drawn.
    pub fn set_center(&mut self, center: Vector) {
        self.center = center;
    }

    /// Angle at which to begin conic gradient, in radians.
    pub fn angle(&self) -> f64 {
        self.angle
    }

    /// Sets angle at which to begin conic gradient, in radians.
    pub fn set_angle(&mut self, angle: f64) {
        self.angle = angle % consts::TAU;
    }

    /// Smoothness of conic gradient ranging from 0.0 to 1.0.
    ///
    /// Completely smooth gradient (with `smoothness` = 1.0) changes color every pixel and
    /// *ignores* pattern of mosaic.
    ///
    /// In contrast, step gradient (with `smoothness` = 0.0) changes its color every
    /// key point of mosaic. Since every mosaic fragment contains a key point then step gradient
    /// changes color once per mosaic fragment.
    ///
    /// Values of `smoothness` between 0.0 and 1.0 can give interesting and even
    /// surprising results.
    ///
    /// # Examples
    ///
    /// ```
    /// use palette::LinSrgb;
    /// use starry_mosaic::{coloring_method::ConicGradient, Vector};
    ///
    /// let gradient = vec![
    ///     (0.2, LinSrgb::new(1.0f64, 0.0, 0.0)),
    ///     (0.4, LinSrgb::new(1.0f64, 1.0, 0.0)),
    ///     (0.6, LinSrgb::new(0.0f64, 1.0, 0.0)),
    ///     (0.8, LinSrgb::new(0.0f64, 0.0, 1.0)),
    /// ];
    /// let conic_smooth_gradient = ConicGradient::new_smooth(
    ///     gradient.clone(),
    ///     Vector::new(100.0, 100.0),
    ///     0.0,
    /// );
    /// let conic_step_gradient = ConicGradient::new_step(
    ///     gradient,
    ///     Vector::new(100.0, 100.0),
    ///     0.0,
    /// );
    ///
    /// assert_eq!(conic_smooth_gradient.smoothness(), 1.0);
    /// assert_eq!(conic_step_gradient.smoothness(), 0.0);
    /// ```
    pub fn smoothness(&self) -> f64 {
        self.smoothness
    }

    /// Sets smoothness of conic gradient (ranging from 0.0 to 1.0).
    pub fn set_smoothness(&mut self, smoothness: f64) {
        self.smoothness = smoothness.clamp(0.0, 1.0);
    }
}

impl<Color> ColoringMethod<Color> for ConicGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    fn interpolate(&self, point: Vector, key_point: Vector) -> Color {
        let smoothed_point = key_point.interpolate(point, self.smoothness);
        let point_vector = smoothed_point - self.center;
        let angle = point_vector.y.atan2(point_vector.x) - self.angle;
        let clamped_angle = (angle + consts::TAU) % consts::TAU;
        self.gradient.get(clamped_angle / consts::TAU)
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
        let key_point = Vector::new(100.0, 150.0);
        assert_eq!(
            conic_gradient.interpolate(Vector::new(150.0, 150.0), key_point),
            gradient.get(0.0)
        );
        assert_eq!(
            conic_gradient.interpolate(Vector::new(100.0, 150.0), key_point),
            gradient.get(0.125)
        );
        assert_eq!(
            conic_gradient.interpolate(Vector::new(50.0, 150.0), key_point),
            gradient.get(0.25)
        );
        let key_point = Vector::new(100.0, 50.0);
        assert_eq!(
            conic_gradient.interpolate(Vector::new(50.0, 50.0), key_point),
            gradient.get(0.5)
        );
        assert_eq!(
            conic_gradient.interpolate(Vector::new(100.0, 50.0), key_point),
            gradient.get(0.625)
        );
        assert_eq!(
            conic_gradient.interpolate(Vector::new(150.0, 50.0), key_point),
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
        let key_point = Vector::new(150.0, 150.0);
        assert_eq!(
            conic_gradient.interpolate(Vector::new(150.0, 150.0), key_point),
            gradient.get(0.25)
        );
        assert_eq!(
            conic_gradient.interpolate(Vector::new(100.0, 150.0), key_point),
            gradient.get(0.25)
        );
        assert_eq!(
            conic_gradient.interpolate(Vector::new(50.0, 150.0), key_point),
            gradient.get(0.25)
        );
        let key_point = Vector::new(50.0, 50.0);
        assert_eq!(
            conic_gradient.interpolate(Vector::new(50.0, 50.0), key_point),
            gradient.get(0.75)
        );
        assert_eq!(
            conic_gradient.interpolate(Vector::new(100.0, 50.0), key_point),
            gradient.get(0.75)
        );
        assert_eq!(
            conic_gradient.interpolate(Vector::new(150.0, 50.0), key_point),
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
        let key_point = Vector::new(150.0, 100.0);
        assert_eq!(
            conic_gradient.interpolate(Vector::new(100.0, 150.0), key_point),
            gradient.get(0.0)
        );
        assert_eq!(
            conic_gradient.interpolate(Vector::new(100.0, 50.0), key_point),
            gradient.get(0.75)
        );
        let key_point = Vector::new(50.0, 100.0);
        assert_eq!(
            conic_gradient.interpolate(Vector::new(100.0, 150.0), key_point),
            gradient.get(0.25)
        );
        assert_eq!(
            conic_gradient.interpolate(Vector::new(100.0, 50.0), key_point),
            gradient.get(0.5)
        );
    }
    #[test]
    fn interpolate_at_center() {
        let gradient = tests::create_lch_gradient();
        let conic_gradient =
            ConicGradient::new_smooth(gradient.clone(), Vector::new(100.0, 100.0), 0.0);
        assert_eq!(
            conic_gradient.interpolate(conic_gradient.center(), conic_gradient.center()),
            gradient.get(0.0)
        );
    }
}
