use palette::{Gradient, Mix};

use super::{super::utility, ColoringMethod, Vector};

/// Defines radial gradient for painting mosaic images.
#[derive(Clone, Debug)]
pub struct RadialGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    gradient: Gradient<Color>,
    inner_center: Vector,
    direction: Vector,
    direction_squared_length: f64,
    inner_radius: f64,
    radius_difference: f64,
    smoothness: f64,
}

impl<Color> RadialGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    /// Creates radial gradient using sizes and positions of two circles.
    ///
    /// # Arguments
    ///
    /// * `gradient`: list of colors or colors stops of gradient.
    /// * `inner_center`: center of inner circle.
    /// * `inner_radius`: radius of inner circle; must be non-negative.
    /// * `outer_center`: center of outer circle.
    /// * `outer_radius`: radius of outer circle; must be non-negative.
    /// If the inner circle is not inside the outer circle then radius of the outer circle
    /// will be increased automatically.
    /// * `smoothness`: smoothness of gradient ranging from 0.0 to 1.0;
    /// see [`RadialGradient::smoothness`] for more information.
    ///
    /// returns: [`RadialGradient<Color>`] - radial gradient initialized with two specified
    /// circles; if these circles are equal returns radial step gradient.
    ///
    /// # Examples
    ///
    /// Next example creates radial semi-step gradient.
    ///
    /// ```
    /// use palette::Lch;
    /// use starry_mosaic::{coloring_method::{ColoringMethod, RadialGradient}, Vector};
    ///
    /// let gradient = vec![
    ///     (0.0, Lch::new(50.0f64, 100.0, 40.0)),
    ///     (0.5, Lch::new(90.0f64, 110.0, 130.0)),
    ///     (1.0, Lch::new(30.0f64, 70.0, 330.0)),
    /// ];
    /// let radial_semi_step_gradient = RadialGradient::new(
    ///     gradient,
    ///     Vector::new(200.0, 100.0),
    ///     50.0,
    ///     Vector::new(200.0, 200.0),
    ///     200.0,
    ///     0.5,
    /// );
    ///
    /// let key_point = Vector::new(200.0, 200.0);
    /// assert_eq!(
    ///     radial_semi_step_gradient.interpolate(&Vector::new(200.0, 300.0), &key_point),
    ///     Lch::new(82.0f64, 108.0, 112.0),
    /// );
    /// ```
    pub fn new<ColorGradient>(
        gradient: ColorGradient,
        inner_center: Vector,
        inner_radius: f64,
        outer_center: Vector,
        outer_radius: f64,
        smoothness: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        let direction = &outer_center - &inner_center;
        let direction_squared_length = direction.squared_length();
        let mut radial_gradient = Self {
            gradient: gradient.into(),
            inner_center,
            direction,
            direction_squared_length,
            inner_radius: inner_radius.max(0.0),
            radius_difference: outer_radius.max(0.0) - inner_radius.max(0.0),
            smoothness: smoothness.clamp(0.0, 1.0),
        };
        radial_gradient.fit_inner_circle_into_outer();
        radial_gradient
    }

    /// Creates radial smooth gradient using sizes and positions of two circles.
    ///
    /// # Arguments
    ///
    /// * `gradient`: list of colors or colors stops of gradient.
    /// * `inner_center`: center of inner circle.
    /// * `inner_radius`: radius of inner circle; must be non-negative.
    /// * `outer_center`: center of outer circle.
    /// * `outer_radius`: radius of outer circle; must be non-negative.
    /// If the inner circle is not inside the outer circle then radius of the outer circle
    /// will be increased automatically.
    ///
    /// returns: [`RadialGradient<Color>`] - radial smooth gradient initialized with two specified
    /// circles; if these circles are equal returns radial step gradient.
    ///
    /// # See also
    ///
    /// * [`RadialGradient::new`].
    /// * [`RadialGradient::smoothness`].
    ///
    /// # Examples
    ///
    /// Next example creates radial smooth gradient.
    ///
    /// ```
    /// use palette::Lch;
    /// use starry_mosaic::{coloring_method::{ColoringMethod, RadialGradient}, Vector};
    ///
    /// let gradient = vec![
    ///     (0.0, Lch::new(50.0f64, 100.0, 40.0)),
    ///     (0.5, Lch::new(90.0f64, 110.0, 130.0)),
    ///     (1.0, Lch::new(30.0f64, 70.0, 330.0)),
    /// ];
    /// let radial_smooth_gradient = RadialGradient::new_smooth(
    ///     gradient,
    ///     Vector::new(200.0, 100.0),
    ///     50.0,
    ///     Vector::new(200.0, 200.0),
    ///     200.0,
    /// );
    ///
    /// let key_point = Vector::new(200.0, 200.0);
    /// assert_eq!(
    ///     radial_smooth_gradient.interpolate(&Vector::new(200.0, 300.0), &key_point),
    ///     Lch::new(78.0f64, 102.0, 98.0),
    /// );
    /// ```
    #[inline(always)]
    pub fn new_smooth<ColorGradient>(
        gradient: ColorGradient,
        inner_center: Vector,
        inner_radius: f64,
        outer_center: Vector,
        outer_radius: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new(
            gradient,
            inner_center,
            inner_radius,
            outer_center,
            outer_radius,
            1.0,
        )
    }

    /// Creates radial step gradient using sizes and positions of two circles.
    ///
    /// # Arguments
    ///
    /// * `gradient`: list of colors or colors stops of gradient.
    /// * `inner_center`: center of inner circle.
    /// * `inner_radius`: radius of inner circle; must be non-negative.
    /// * `outer_center`: center of outer circle.
    /// * `outer_radius`: radius of outer circle; must be non-negative.
    /// If the inner circle is not inside the outer circle then radius of the outer circle
    /// will be increased automatically.
    ///
    /// returns: [`RadialGradient<Color>`] - radial step gradient initialized with two specified
    /// circles.
    ///
    /// # See also
    ///
    /// * [`RadialGradient::new`].
    /// * [`RadialGradient::smoothness`].
    ///
    /// # Examples
    ///
    /// Next example creates radial step gradient.
    ///
    /// ```
    /// use palette::Lch;
    /// use starry_mosaic::{coloring_method::{ColoringMethod, RadialGradient}, Vector};
    ///
    /// let gradient = vec![
    ///     (0.0, Lch::new(50.0f64, 100.0, 40.0)),
    ///     (0.5, Lch::new(90.0f64, 110.0, 130.0)),
    ///     (1.0, Lch::new(30.0f64, 70.0, 330.0)),
    /// ];
    /// let radial_step_gradient = RadialGradient::new_step(
    ///     gradient,
    ///     Vector::new(200.0, 100.0),
    ///     50.0,
    ///     Vector::new(200.0, 200.0),
    ///     200.0,
    /// );
    ///
    /// let key_point = Vector::new(200.0, 200.0);
    /// assert_eq!(
    ///     radial_step_gradient.interpolate(&Vector::new(200.0, 300.0), &key_point),
    ///     Lch::new(66.0f64, 104.0, 76.0),
    /// );
    /// ```
    #[inline(always)]
    pub fn new_step<ColorGradient>(
        gradient: ColorGradient,
        inner_center: Vector,
        inner_radius: f64,
        outer_center: Vector,
        outer_radius: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new(
            gradient,
            inner_center,
            inner_radius,
            outer_center,
            outer_radius,
            0.0,
        )
    }

    /// Creates radial simple gradient using size and position of circle that bounds it.
    ///
    /// Simple gradient is a regular radial gradient with the center of the inner circle matching
    /// the center of the outer circle and the radius of the inner circle being 0.0.
    ///
    /// # Arguments
    ///
    /// * `gradient`: list of colors or colors stops of gradient.
    /// * `center`: center of circle that bounds radial gradient.
    /// * `radius`: radius of circle; must be non-negative.
    /// * `smoothness`: smoothness of gradient ranging from 0.0 to 1.0;
    /// see [`RadialGradient::smoothness`] for more information.
    ///
    /// returns: [`RadialGradient<Color>`] - radial simple gradient initialized with single circle.
    ///
    /// # See also
    ///
    /// * [`RadialGradient::new`].
    ///
    /// # Examples
    ///
    /// Next example creates radial simple semi-step gradient.
    ///
    /// ```
    /// use palette::Lch;
    /// use starry_mosaic::{coloring_method::{ColoringMethod, RadialGradient}, Vector};
    ///
    /// let gradient = vec![
    ///     (0.0, Lch::new(50.0f64, 100.0, 40.0)),
    ///     (0.5, Lch::new(90.0f64, 110.0, 130.0)),
    ///     (1.0, Lch::new(30.0f64, 70.0, 330.0)),
    /// ];
    /// let radial_simple_semi_step_gradient = RadialGradient::new_simple(
    ///     gradient,
    ///     Vector::new(200.0, 200.0),
    ///     200.0,
    ///     0.5,
    /// );
    ///
    /// let key_point = Vector::new(200.0, 200.0);
    /// assert_eq!(
    ///     radial_simple_semi_step_gradient.interpolate(&Vector::new(200.0, 300.0), &key_point),
    ///     Lch::new(70.0f64, 105.0, 85.0),
    /// );
    /// ```
    #[inline(always)]
    pub fn new_simple<ColorGradient>(
        gradient: ColorGradient,
        center: Vector,
        radius: f64,
        smoothness: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new(gradient, center.clone(), 0.0, center, radius, smoothness)
    }

    /// Creates radial simple smooth gradient using size and position of circle that bounds it.
    ///
    /// Simple gradient is a regular radial gradient with the center of the inner circle matching
    /// the center of the outer circle and the radius of the inner circle being 0.0.
    ///
    /// # Arguments
    ///
    /// * `gradient`: list of colors or colors stops of gradient.
    /// * `center`: center of circle that bounds radial gradient.
    /// * `radius`: radius of circle; must be non-negative.
    ///
    /// returns: [`RadialGradient<Color>`] - radial simple smooth gradient initialized with
    /// single circle.
    ///
    /// # See also
    ///
    /// * [`RadialGradient::new`].
    /// * [`RadialGradient::new_simple`].
    /// * [`RadialGradient::smoothness`].
    ///
    /// # Examples
    ///
    /// Next example creates radial simple smooth gradient.
    ///
    /// ```
    /// use palette::Lch;
    /// use starry_mosaic::{coloring_method::{ColoringMethod, RadialGradient}, Vector};
    ///
    /// let gradient = vec![
    ///     (0.0, Lch::new(50.0f64, 100.0, 40.0)),
    ///     (0.5, Lch::new(90.0f64, 110.0, 130.0)),
    ///     (1.0, Lch::new(30.0f64, 70.0, 330.0)),
    /// ];
    /// let radial_simple_smooth_gradient = RadialGradient::new_simple_smooth(
    ///     gradient,
    ///     Vector::new(200.0, 200.0),
    ///     200.0,
    /// );
    ///
    /// let key_point = Vector::new(200.0, 200.0);
    /// assert_eq!(
    ///     radial_simple_smooth_gradient.interpolate(&Vector::new(200.0, 300.0), &key_point),
    ///     Lch::new(90.0f64, 110.0, 130.0),
    /// );
    /// ```
    #[inline(always)]
    pub fn new_simple_smooth<ColorGradient>(
        gradient: ColorGradient,
        center: Vector,
        radius: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new_simple(gradient, center, radius, 1.0)
    }

    /// Creates radial simple step gradient using size and position of circle that bounds it.
    ///
    /// Simple gradient is a regular radial gradient with the center of the inner circle matching
    /// the center of the outer circle and the radius of the inner circle being 0.0.
    ///
    /// # Arguments
    ///
    /// * `gradient`: list of colors or colors stops of gradient.
    /// * `center`: center of circle that bounds radial gradient.
    /// * `radius`: radius of circle; must be non-negative.
    ///
    /// returns: [`RadialGradient<Color>`] - radial simple step gradient initialized with
    /// single circle.
    ///
    /// # See also
    ///
    /// * [`RadialGradient::new`].
    /// * [`RadialGradient::new_simple`].
    /// * [`RadialGradient::smoothness`].
    ///
    /// # Examples
    ///
    /// Next example creates radial simple step gradient.
    ///
    /// ```
    /// use palette::Lch;
    /// use starry_mosaic::{coloring_method::{ColoringMethod, RadialGradient}, Vector};
    ///
    /// let gradient = vec![
    ///     (0.0, Lch::new(50.0f64, 100.0, 40.0)),
    ///     (0.5, Lch::new(90.0f64, 110.0, 130.0)),
    ///     (1.0, Lch::new(30.0f64, 70.0, 330.0)),
    /// ];
    /// let radial_simple_step_gradient = RadialGradient::new_simple_step(
    ///     gradient,
    ///     Vector::new(200.0, 200.0),
    ///     200.0,
    /// );
    ///
    /// let key_point = Vector::new(200.0, 200.0);
    /// assert_eq!(
    ///     radial_simple_step_gradient.interpolate(&Vector::new(200.0, 300.0), &key_point),
    ///     Lch::new(50.0f64, 100.0, 40.0),
    /// );
    /// ```
    #[inline(always)]
    pub fn new_simple_step<ColorGradient>(
        gradient: ColorGradient,
        center: Vector,
        radius: f64,
    ) -> Self
    where
        ColorGradient: Into<Gradient<Color>>,
    {
        Self::new_simple(gradient, center, radius, 0.0)
    }

    /// Center of inner circle of radial gradient.
    pub fn inner_center(&self) -> Vector {
        self.inner_center.clone()
    }

    /// Sets center of inner circle of radial gradient.
    ///
    /// If the inner circle is not inside the outer circle then radius of the outer circle
    /// will be increased automatically.
    ///
    /// # Arguments
    ///
    /// * `inner_center`: center of inner circle.
    pub fn set_inner_center(&mut self, inner_center: Vector) {
        let outer_center = &self.inner_center + &self.direction;
        self.inner_center = inner_center;
        self.set_outer_center(outer_center);
    }

    /// Radius of inner circle of radial gradient.
    pub fn inner_radius(&self) -> f64 {
        self.inner_radius
    }

    /// Sets radius of inner circle of radial gradient.
    ///
    /// If the inner circle is not inside the outer circle then radius of the outer circle
    /// will be increased automatically.
    ///
    /// # Arguments
    ///
    /// * `inner_radius`: radius of inner circle; must be non-negative.
    pub fn set_inner_radius(&mut self, inner_radius: f64) {
        let outer_radius = self.inner_radius + self.radius_difference;
        self.inner_radius = inner_radius.max(0.0);
        self.set_outer_radius(outer_radius);
    }

    /// Center of outer circle that bounds radial gradient.
    pub fn outer_center(&self) -> Vector {
        &self.inner_center + &self.direction
    }

    /// Sets center of outer circle that bounds radial gradient.
    ///
    /// If the inner circle is not inside the outer circle then radius of the outer circle
    /// will be increased automatically.
    ///
    /// # Arguments
    ///
    /// * `outer_center`: center of outer circle.
    pub fn set_outer_center(&mut self, outer_center: Vector) {
        self.direction = &outer_center - &self.inner_center;
        self.direction_squared_length = self.direction.squared_length();
        self.fit_inner_circle_into_outer();
    }

    /// Radius of outer circle that bounds radial gradient.
    pub fn outer_radius(&self) -> f64 {
        self.inner_radius + self.radius_difference
    }

    /// Sets radius of outer circle that bounds radial gradient.
    ///
    /// If the inner circle is not inside the outer circle then radius of the outer circle
    /// will be increased automatically.
    ///
    /// # Arguments
    ///
    /// * `outer_radius`: radius of outer circle; must be non-negative.
    pub fn set_outer_radius(&mut self, outer_radius: f64) {
        self.radius_difference = outer_radius.max(0.0) - self.inner_radius;
        self.fit_inner_circle_into_outer();
    }

    /// Smoothness of radial gradient ranging from 0.0 to 1.0.
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
    /// use starry_mosaic::{coloring_method::RadialGradient, Vector};
    ///
    /// let gradient = vec![
    ///     (0.2, LinSrgb::new(1.0f64, 0.0, 0.0)),
    ///     (0.4, LinSrgb::new(1.0f64, 1.0, 0.0)),
    ///     (0.6, LinSrgb::new(0.0f64, 1.0, 0.0)),
    ///     (0.8, LinSrgb::new(0.0f64, 0.0, 1.0)),
    /// ];
    /// let radial_smooth_gradient = RadialGradient::new_smooth(
    ///     gradient.clone(),
    ///     Vector::new(200.0, 100.0),
    ///     50.0,
    ///     Vector::new(200.0, 200.0),
    ///     200.0,
    /// );
    /// let radial_step_gradient = RadialGradient::new_step(
    ///     gradient,
    ///     Vector::new(200.0, 100.0),
    ///     50.0,
    ///     Vector::new(200.0, 200.0),
    ///     200.0,
    /// );
    ///
    /// assert_eq!(radial_smooth_gradient.smoothness(), 1.0);
    /// assert_eq!(radial_step_gradient.smoothness(), 0.0);
    /// ```
    pub fn smoothness(&self) -> f64 {
        self.smoothness
    }

    /// Sets smoothness of radial gradient (ranging from 0.0 to 1.0).
    pub fn set_smoothness(&mut self, smoothness: f64) {
        self.smoothness = smoothness.clamp(0.0, 1.0);
    }

    #[inline(always)]
    fn fit_inner_circle_into_outer(&mut self) {
        self.radius_difference = self
            .radius_difference
            .max(self.direction.length() + utility::EPSILON * 2.0);
    }
}

impl<Color> ColoringMethod<Color> for RadialGradient<Color>
where
    Color: Mix<Scalar = f64> + Clone,
{
    fn interpolate(&self, point: &Vector, key_point: &Vector) -> Color {
        let smoothed_point = key_point.interpolate(point, self.smoothness);
        let point_vector = &smoothed_point - &self.inner_center;
        let alpha = self.direction_squared_length - self.radius_difference.powi(2);
        let beta = point_vector.dot(&self.direction) + self.inner_radius * self.radius_difference;
        let gamma = point_vector.squared_length() - self.inner_radius.powi(2);
        let discriminant = beta * beta - alpha * gamma;
        let interpolation_factor = (beta - discriminant.sqrt()) / alpha;
        self.gradient.get(interpolation_factor)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::tests, *};

    #[test]
    fn set_inner_center() {
        let gradient = tests::create_rgb_gradient();
        let mut radial_gradient = RadialGradient::new_smooth(
            gradient,
            Vector::new(150.0, 250.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        radial_gradient.set_inner_center(Vector::new(50.0, 250.0));
        assert!(radial_gradient.radius_difference > radial_gradient.direction.length());
        assert!(radial_gradient.outer_radius() > 200.0);
    }
    #[test]
    fn set_inner_radius() {
        let gradient = tests::create_rgb_gradient();
        let mut radial_gradient = RadialGradient::new_smooth(
            gradient,
            Vector::new(150.0, 250.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        radial_gradient.set_inner_radius(150.0);
        assert!(radial_gradient.radius_difference > radial_gradient.direction.length());
        assert!(radial_gradient.outer_radius() > 200.0);
    }
    #[test]
    fn set_negative_inner_radius() {
        let gradient = tests::create_rgb_gradient();
        let mut radial_gradient = RadialGradient::new_smooth(
            gradient,
            Vector::new(150.0, 250.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        radial_gradient.set_inner_radius(-150.0);
        assert!(radial_gradient.radius_difference > radial_gradient.direction.length());
        assert_eq!(radial_gradient.inner_radius, 0.0);
    }
    #[test]
    fn set_outer_center() {
        let gradient = tests::create_rgb_gradient();
        let mut radial_gradient = RadialGradient::new_smooth(
            gradient,
            Vector::new(150.0, 250.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        radial_gradient.set_outer_center(Vector::new(350.0, 250.0));
        assert!(radial_gradient.radius_difference > radial_gradient.direction.length());
        assert!(radial_gradient.outer_radius() > 200.0);
    }
    #[test]
    fn set_outer_radius() {
        let gradient = tests::create_rgb_gradient();
        let mut radial_gradient = RadialGradient::new_smooth(
            gradient,
            Vector::new(150.0, 250.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        radial_gradient.set_outer_radius(100.0);
        assert!(radial_gradient.radius_difference > radial_gradient.direction.length());
        assert!(radial_gradient.outer_radius() > 100.0);
    }
    #[test]
    fn set_negative_outer_radius() {
        let gradient = tests::create_rgb_gradient();
        let mut radial_gradient = RadialGradient::new_smooth(
            gradient,
            Vector::new(150.0, 250.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        radial_gradient.set_outer_radius(-100.0);
        assert!(radial_gradient.radius_difference > radial_gradient.direction.length());
        assert!(radial_gradient.outer_radius() > 150.0);
    }
    #[test]
    fn interpolate_smooth() {
        let gradient = tests::create_rgb_gradient();
        let radial_gradient = RadialGradient::new_smooth(
            gradient.clone(),
            Vector::new(250.0, 150.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        let key_point = Vector::new(250.0, 325.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(250.0, 200.0 + index * 50.0);
            assert_eq!(
                radial_gradient.interpolate(&point, &key_point),
                gradient.get(index / 5.0)
            );
        }
        let key_point = Vector::new(250.0, 75.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(250.0, 100.0 - index * 10.0);
            assert_eq!(
                radial_gradient.interpolate(&point, &key_point),
                gradient.get(index / 5.0)
            );
        }
    }
    #[test]
    fn interpolate_step() {
        let gradient = tests::create_lch_gradient();
        let radial_gradient = RadialGradient::new_step(
            gradient.clone(),
            Vector::new(250.0, 150.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        let key_point = Vector::new(250.0, 325.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(250.0, 200.0 + index * 50.0);
            assert_eq!(
                radial_gradient.interpolate(&point, &key_point),
                gradient.get(0.5)
            );
        }
        let key_point = Vector::new(250.0, 75.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(250.0, 100.0 - index * 10.0);
            assert_eq!(
                radial_gradient.interpolate(&point, &key_point),
                gradient.get(0.5)
            );
        }
    }
    #[test]
    fn interpolate_semi_step() {
        let gradient = tests::create_hsl_gradient();
        let radial_gradient = RadialGradient::new(
            gradient.clone(),
            Vector::new(250.0, 150.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
            0.5,
        );
        let key_point = Vector::new(250.0, 325.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(250.0, 200.0 + index * 50.0);
            assert_eq!(
                radial_gradient.interpolate(&point, &key_point),
                gradient.get(0.25 + index / 10.0)
            );
        }
        let key_point = Vector::new(250.0, 75.0);
        for index in 0..=5 {
            let index = index as f64;
            let point = Vector::new(250.0, 100.0 - index * 10.0);
            assert_eq!(
                radial_gradient.interpolate(&point, &key_point),
                gradient.get(0.25 + index / 10.0)
            );
        }
    }
    #[test]
    fn interpolate_center_position() {
        let gradient = tests::create_hsl_gradient();
        let radial_gradient = RadialGradient::new_smooth(
            gradient.clone(),
            Vector::new(150.0, 250.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        assert_eq!(
            radial_gradient
                .interpolate(&radial_gradient.inner_center, &radial_gradient.inner_center),
            gradient.get(0.0)
        );
    }
    #[test]
    fn interpolate_edge_positions() {
        let gradient = tests::create_lch_gradient();
        let radial_gradient = RadialGradient::new_smooth(
            gradient.clone(),
            Vector::new(350.0, 250.0),
            50.0,
            Vector::new(250.0, 250.0),
            200.0,
        );
        let point = Vector::new(0.0, 250.0);
        assert_eq!(
            radial_gradient.interpolate(&point, &radial_gradient.inner_center),
            gradient.get(1.0)
        );
        let point = Vector::new(500.0, 250.0);
        assert_eq!(
            radial_gradient.interpolate(&point, &radial_gradient.inner_center),
            gradient.get(1.0)
        );
        let point = Vector::new(250.0, 0.0);
        assert_eq!(
            radial_gradient.interpolate(&point, &radial_gradient.inner_center),
            gradient.get(1.0)
        );
        let point = Vector::new(250.0, 500.0);
        assert_eq!(
            radial_gradient.interpolate(&point, &radial_gradient.inner_center),
            gradient.get(1.0)
        );
    }
}
