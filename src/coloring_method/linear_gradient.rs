use palette::{Gradient, Mix};

use super::{super::utility, ColoringMethod, Vector};

/// Defines linear gradient for painting mosaic images.
#[derive(Clone, Debug)]
pub struct LinearGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    gradient: Gradient<Color>,
    start_point: Vector,
    direction: Vector,
    direction_squared_length: f64,
    smoothness: f64,
}

impl<Color> LinearGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    /// Creates linear gradient along the line connecting two given points.
    ///
    /// # Arguments
    ///
    /// * `gradient`: list of colors or colors stops of gradient.
    /// * `start_point`: starting point of line along which the gradient is drawn.
    /// * `end_point`: end point of line along which the gradient is drawn.
    /// * `smoothness`: smoothness of gradient ranging from 0.0 to 1.0;
    /// see [`LinearGradient::smoothness`] for more information.
    ///
    /// returns: [`LinearGradient<Color>`] - linear gradient along the line connecting two points;
    /// if these points are equal returns horizontal step gradient.
    ///
    /// # Examples
    ///
    /// Next example creates linear semi-step gradient from (0.0, 0.0) to (100.0, 100.0).
    ///
    /// ```
    /// use palette::Hsl;
    /// use starry_mosaic::{coloring_method::{ColoringMethod, LinearGradient}, Vector};
    ///
    /// let gradient = vec![
    ///     (0.1, Hsl::new(0.0f64, 1.0, 0.5)),
    ///     (0.5, Hsl::new(120.0f64, 1.0, 0.5)),
    ///     (0.9, Hsl::new(240.0f64, 1.0, 0.5)),
    /// ];
    /// let start_point = Vector::new(0.0, 0.0);
    /// let end_point = Vector::new(100.0, 100.0);
    /// let linear_semi_step_gradient = LinearGradient::new(gradient, start_point, end_point, 0.4);
    ///
    /// let key_point = Vector::new(50.0, 50.0);
    /// assert_eq!(
    ///     linear_semi_step_gradient.interpolate(&Vector::new(0.0, 20.0), &key_point),
    ///     Hsl::new(72.0f64, 1.0, 0.5)
    /// );
    /// ```
    pub fn new<ColorGradient>(
        gradient: ColorGradient,
        start_point: Vector,
        end_point: Vector,
        smoothness: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        let direction = &end_point - &start_point;
        let direction_squared_length = direction.squared_length();
        let mut linear_gradient = Self {
            gradient: gradient.into(),
            start_point,
            direction,
            direction_squared_length,
            smoothness: smoothness.clamp(0.0, 1.0),
        };
        linear_gradient.set_end_point(end_point);
        linear_gradient
    }

    /// Creates linear smooth gradient along the line connecting two given points.
    ///
    /// # Arguments
    ///
    /// * `gradient`: list of colors or colors stops of gradient.
    /// * `start_point`: starting point of line along which the gradient is drawn.
    /// * `end_point`: end point of line along which the gradient is drawn.
    ///
    /// returns: [`LinearGradient<Color>`] - linear smooth gradient along the line connecting
    /// two points; if these points are equal returns horizontal step gradient.
    ///
    /// # See also
    ///
    /// * [`LinearGradient::new`].
    /// * [`LinearGradient::smoothness`].
    ///
    /// # Examples
    ///
    /// Next example creates linear smooth gradient from (0.0, 0.0) to (100.0, 100.0).
    ///
    /// ```
    /// use palette::Hsl;
    /// use starry_mosaic::{coloring_method::{ColoringMethod, LinearGradient}, Vector};
    ///
    /// let gradient = vec![
    ///     (0.1, Hsl::new(0.0f64, 1.0, 0.5)),
    ///     (0.5, Hsl::new(120.0f64, 1.0, 0.5)),
    ///     (0.9, Hsl::new(240.0f64, 1.0, 0.5)),
    /// ];
    /// let start_point = Vector::new(0.0, 0.0);
    /// let end_point = Vector::new(100.0, 100.0);
    /// let linear_smooth_gradient = LinearGradient::new(gradient, start_point, end_point, 1.0);
    ///
    /// let key_point = Vector::new(50.0, 50.0);
    /// assert_eq!(
    ///     linear_smooth_gradient.interpolate(&Vector::new(0.0, 20.0), &key_point),
    ///     Hsl::new(0.0f64, 1.0, 0.5)
    /// );
    /// ```
    #[inline(always)]
    pub fn new_smooth<ColorGradient>(
        gradient: ColorGradient,
        start_point: Vector,
        end_point: Vector,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new(gradient, start_point, end_point, 1.0)
    }

    /// Creates linear step gradient along the line connecting two given points.
    ///
    /// # Arguments
    ///
    /// * `gradient`: list of colors or colors stops of gradient.
    /// * `start_point`: starting point of line along which the gradient is drawn.
    /// * `end_point`: end point of line along which the gradient is drawn.
    ///
    /// returns: [`LinearGradient<Color>`] - linear step gradient along the line connecting
    /// two points; if these points are equal returns horizontal step gradient.
    ///
    /// # See also
    ///
    /// * [`LinearGradient::new`].
    /// * [`LinearGradient::smoothness`].
    ///
    /// # Examples
    ///
    /// Next example creates linear step gradient from (0.0, 0.0) to (100.0, 100.0).
    ///
    /// ```
    /// use palette::Hsl;
    /// use starry_mosaic::{coloring_method::{ColoringMethod, LinearGradient}, Vector};
    ///
    /// let gradient = vec![
    ///     (0.1, Hsl::new(0.0f64, 1.0, 0.5)),
    ///     (0.5, Hsl::new(120.0f64, 1.0, 0.5)),
    ///     (0.9, Hsl::new(240.0f64, 1.0, 0.5)),
    /// ];
    /// let start_point = Vector::new(0.0, 0.0);
    /// let end_point = Vector::new(100.0, 100.0);
    /// let linear_step_gradient = LinearGradient::new(gradient, start_point, end_point, 0.0);
    ///
    /// let key_point = Vector::new(50.0, 50.0);
    /// assert_eq!(
    ///     linear_step_gradient.interpolate(&Vector::new(0.0, 20.0), &key_point),
    ///     Hsl::new(120.0f64, 1.0, 0.5)
    /// );
    /// ```
    #[inline(always)]
    pub fn new_step<ColorGradient>(
        gradient: ColorGradient,
        start_point: Vector,
        end_point: Vector,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new(gradient, start_point, end_point, 0.0)
    }

    /// Starting point of line along which linear gradient is drawn.
    pub fn start_point(&self) -> Vector {
        self.start_point.clone()
    }

    /// Sets starting point of line along which linear gradient is drawn.
    pub fn set_start_point(&mut self, start_point: Vector) {
        let end_point = &self.start_point + &self.direction;
        self.start_point = start_point;
        self.set_direction(end_point);
    }

    /// End point of line along which linear gradient is drawn.
    pub fn end_point(&self) -> Vector {
        &self.start_point + &self.direction
    }

    /// Sets end point of line along which linear gradient is drawn.
    pub fn set_end_point(&mut self, end_point: Vector) {
        self.set_direction(end_point);
    }

    /// Smoothness of linear gradient ranging from 0.0 to 1.0.
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
    /// use starry_mosaic::{coloring_method::LinearGradient, Vector};
    ///
    /// let gradient = vec![
    ///     (0.2, LinSrgb::new(1.0f64, 0.0, 0.0)),
    ///     (0.4, LinSrgb::new(1.0f64, 1.0, 0.0)),
    ///     (0.6, LinSrgb::new(0.0f64, 1.0, 0.0)),
    ///     (0.8, LinSrgb::new(0.0f64, 0.0, 1.0)),
    /// ];
    /// let linear_smooth_gradient = LinearGradient::new_smooth(
    ///     gradient.clone(),
    ///     Vector::new(100.0, 100.0),
    ///     Vector::new(0.0, 50.0),
    /// );
    /// let linear_step_gradient = LinearGradient::new_step(
    ///     gradient,
    ///     Vector::new(100.0, 100.0),
    ///     Vector::new(0.0, 50.0),
    /// );
    ///
    /// assert_eq!(linear_smooth_gradient.smoothness(), 1.0);
    /// assert_eq!(linear_step_gradient.smoothness(), 0.0);
    /// ```
    pub fn smoothness(&self) -> f64 {
        self.smoothness
    }

    /// Sets smoothness of linear gradient (ranging from 0.0 to 1.0).
    pub fn set_smoothness(&mut self, smoothness: f64) {
        self.smoothness = smoothness.clamp(0.0, 1.0);
    }

    #[inline(always)]
    fn set_direction(&mut self, end_point: Vector) {
        self.direction = if self.start_point != end_point {
            &end_point - &self.start_point
        } else {
            Vector::new(utility::EPSILON * 2.0, 0.0)
        };
        self.direction_squared_length = self.direction.squared_length();
    }
}

impl<Color> ColoringMethod<Color> for LinearGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    fn interpolate(&self, point: &Vector, key_point: &Vector) -> Color {
        let smoothed_point = key_point.interpolate(point, self.smoothness);
        let interpolation_factor = (&smoothed_point - &self.start_point).dot(&self.direction)
            / self.direction_squared_length;
        self.gradient.get(interpolation_factor)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::tests, *};

    #[test]
    fn create_with_null_direction_vector() {
        let gradient = tests::create_rgb_gradient();
        let point = Vector::new(100.0, 100.0);
        let linear_gradient = LinearGradient::new_smooth(gradient, point.clone(), point);
        assert!(linear_gradient.direction_squared_length > 0.0);
    }
    #[test]
    fn set_start_point_equal_to_end_point() {
        let gradient = tests::create_hsl_gradient();
        let mut linear_gradient =
            LinearGradient::new_smooth(gradient, Vector::new(0.0, 0.0), Vector::new(100.0, 100.0));
        linear_gradient.set_start_point(linear_gradient.end_point());
        assert!(linear_gradient.direction_squared_length > 0.0);
    }
    #[test]
    fn set_end_point_equal_to_start_point() {
        let gradient = tests::create_lch_gradient();
        let mut linear_gradient =
            LinearGradient::new_smooth(gradient, Vector::new(0.0, 0.0), Vector::new(100.0, 100.0));
        linear_gradient.set_end_point(linear_gradient.start_point());
        assert!(linear_gradient.direction_squared_length > 0.0);
    }
    #[test]
    fn interpolate_smooth() {
        let gradient = tests::create_rgb_gradient();
        let linear_gradient = LinearGradient::new_smooth(
            gradient.clone(),
            Vector::new(0.0, 0.0),
            Vector::new(100.0, 100.0),
        );
        let key_point = Vector::new(25.0, 25.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(index * 10.0, index * 10.0);
            assert_eq!(
                linear_gradient.interpolate(&point, &key_point),
                gradient.get(index / 10.0)
            );
        }
        let key_point = Vector::new(75.0, 75.0);
        for index in 5..=10 {
            let index = index as f64;
            let point = Vector::new(index * 10.0, index * 10.0);
            assert_eq!(
                linear_gradient.interpolate(&point, &key_point),
                gradient.get(index / 10.0)
            );
        }
    }
    #[test]
    fn interpolate_step() {
        let gradient = tests::create_lch_gradient();
        let linear_gradient = LinearGradient::new_step(
            gradient.clone(),
            Vector::new(0.0, 0.0),
            Vector::new(100.0, 100.0),
        );
        let key_point = Vector::new(25.0, 25.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(index * 10.0, index * 10.0);
            assert_eq!(
                linear_gradient.interpolate(&point, &key_point),
                gradient.get(0.25)
            );
        }
        let key_point = Vector::new(75.0, 75.0);
        for index in 5..=10 {
            let index = index as f64;
            let point = Vector::new(index * 10.0, index * 10.0);
            assert_eq!(
                linear_gradient.interpolate(&point, &key_point),
                gradient.get(0.75)
            );
        }
    }
    #[test]
    fn interpolate_semi_step() {
        let gradient = tests::create_hsl_gradient();
        let linear_gradient = LinearGradient::new(
            gradient.clone(),
            Vector::new(0.0, 0.0),
            Vector::new(100.0, 100.0),
            0.5,
        );
        let key_point = Vector::new(25.0, 25.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(index * 10.0, index * 10.0);
            assert_eq!(
                linear_gradient.interpolate(&point, &key_point),
                gradient.get(0.125 + index / 20.0)
            );
        }
        let key_point = Vector::new(75.0, 75.0);
        for index in 5..=10 {
            let index = index as f64;
            let point = Vector::new(index * 10.0, index * 10.0);
            assert_eq!(
                linear_gradient.interpolate(&point, &key_point),
                gradient.get(0.625 + (index - 5.0) / 20.0)
            );
        }
    }
    #[test]
    fn interpolate_with_minimal_distance() {
        let gradient = tests::create_rgb_gradient();
        let start_point = Vector::new(50.0, 50.0);
        let end_point = Vector::new(51.0, 50.0);
        let linear_gradient =
            LinearGradient::new_smooth(gradient.clone(), start_point.clone(), end_point.clone());
        assert_ne!(
            linear_gradient.interpolate(&start_point, &start_point),
            linear_gradient.interpolate(&end_point, &end_point)
        );
    }
}
