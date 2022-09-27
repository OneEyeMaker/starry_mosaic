use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::{utility, vector::Vector};

pub trait TryToTransform: Sized {
    fn try_to_transform(&self, transformation: &Transformation) -> Option<Self>;
}

pub trait Transform: TryToTransform {
    fn transform(&self, transformation: &Transformation) -> Self;
}

impl<Transformable> TryToTransform for Transformable
where
    Transformable: Transform,
{
    #[inline(always)]
    fn try_to_transform(&self, transformation: &Transformation) -> Option<Self> {
        Some(self.transform(transformation))
    }
}

/// Represents 2D transformation.
#[derive(Clone, Debug, Default)]
pub struct Transformation {
    /// Translation (movement) along horizontal and vertical axes.
    pub translation: Vector,

    /// Scale factors along horizontal and vertical axes.
    pub scale: Scale,

    /// Shear (skew) factors along horizontal and vertical axes.
    pub shear: Vector,

    /// Rotation angle in radians.
    pub rotation_angle: f64,
}

impl Transformation {
    /// Constructs transformation based on translation.
    ///
    /// # Arguments
    ///
    /// * `translation`: 2D vector representing translation.
    ///
    /// returns: [`Transformation`] - transformation with set translation (position).
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::{transform::Transformation, Vector};
    ///
    /// let translation = Vector::new(100.0, 50.0);
    /// let transformation = Transformation::from_translation(translation);
    ///
    /// assert_eq!(transformation.translation, translation);
    /// ```
    pub fn from_translation<VectorLike>(translation: VectorLike) -> Self
    where
        VectorLike: Into<Vector>,
    {
        let mut transformation = Transformation::default();
        transformation.translation = translation.into();
        transformation
    }

    /// Constructs transformation based on rotation.
    ///
    /// # Arguments
    ///
    /// * `rotation_angle`: rotation angle in radians.
    ///
    /// returns: [`Transformation`] - transformation with set rotation.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts;
    ///
    /// use starry_mosaic::transform::Transformation;
    ///
    /// let transformation = Transformation::from_rotation(consts::FRAC_PI_6);
    ///
    /// assert_eq!(transformation.rotation_angle, consts::FRAC_PI_6);
    /// ```
    pub fn from_rotation(rotation_angle: f64) -> Self {
        let mut transformation = Transformation::default();
        transformation.rotation_angle = rotation_angle;
        transformation
    }

    /// Constructs transformation based on scale.
    ///
    /// # Arguments
    ///
    /// * `scale`: scale factors along horizontal and vertical axes.
    ///
    /// returns: [`Transformation`] - transformation with set scale.
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::transform::{Scale, Transformation};
    ///
    /// let scale = Scale::new(2.0, 3.0);
    /// let transformation = Transformation::from_scale(scale);
    ///
    /// assert_eq!(transformation.scale, scale);
    /// ```
    pub fn from_scale<ScaleLike>(scale: ScaleLike) -> Self
    where
        ScaleLike: Into<Scale>,
    {
        let mut transformation = Transformation::default();
        transformation.scale = scale.into();
        transformation
    }

    /// Constructs transformation based on shear.
    ///
    /// # Arguments
    ///
    /// * `shear`: shear factors along horizontal and vertical axes.
    ///
    /// returns: [`Transformation`] - transformation with set shear (skew).
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::{transform::Transformation, Vector};
    ///
    /// let shear = Vector::new(1.0, -0.5);
    /// let transformation = Transformation::from_shear(shear);
    ///
    /// assert_eq!(transformation.shear, shear);
    /// ```
    pub fn from_shear<VectorLike>(shear: VectorLike) -> Self
    where
        VectorLike: Into<Vector>,
    {
        let mut transformation = Transformation::default();
        transformation.shear = shear.into();
        transformation
    }

    pub fn try_to_apply<Transformable>(
        &self,
        transformable: &Transformable,
    ) -> Option<Transformable>
    where
        Transformable: TryToTransform,
    {
        transformable.try_to_transform(self)
    }

    pub fn apply<Transformable>(&self, transformable: &Transformable) -> Transformable
    where
        Transformable: Transform,
    {
        transformable.transform(self)
    }
}

impl PartialEq for Transformation {
    fn eq(&self, transformation: &Self) -> bool {
        self.translation == transformation.translation
            && utility::approx_eq(self.rotation_angle, transformation.rotation_angle)
            && self.scale == transformation.scale
            && self.shear == transformation.shear
    }
}

impl Add for Transformation {
    type Output = Transformation;
    fn add(self, transformation: Self) -> Self::Output {
        Transformation {
            translation: self.translation + transformation.translation,
            rotation_angle: self.rotation_angle + transformation.rotation_angle,
            scale: self.scale * transformation.scale,
            shear: self.shear + transformation.shear,
        }
    }
}
impl Sub for Transformation {
    type Output = Transformation;
    fn sub(self, transformation: Self) -> Self::Output {
        Transformation {
            translation: self.translation - transformation.translation,
            rotation_angle: self.rotation_angle - transformation.rotation_angle,
            scale: self.scale / transformation.scale,
            shear: self.shear - transformation.shear,
        }
    }
}

impl Neg for Transformation {
    type Output = Transformation;
    fn neg(self) -> Self::Output {
        Transformation {
            translation: -self.translation,
            rotation_angle: -self.rotation_angle,
            scale: -self.scale,
            shear: -self.shear,
        }
    }
}

impl AddAssign for Transformation {
    fn add_assign(&mut self, transformation: Self) {
        self.translation += transformation.translation;
        self.rotation_angle += transformation.rotation_angle;
        self.scale *= transformation.scale;
        self.shear += transformation.shear;
    }
}
impl SubAssign for Transformation {
    fn sub_assign(&mut self, transformation: Self) {
        self.translation -= transformation.translation;
        self.rotation_angle -= transformation.rotation_angle;
        self.scale /= transformation.scale;
        self.shear -= transformation.shear;
    }
}

/// Represents scale in 2D coordinate space.
#[derive(Clone, Copy, Debug)]
pub struct Scale {
    /// Scale along horizontal (X) axis.
    pub x: f64,

    /// Scale along vertical (Y) axis.
    pub y: f64,
}

impl Scale {
    /// Defines scale along horizontal and vertical axes.
    #[inline(always)]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Defines uniform scale along both (horizontal and vertical) axes.
    #[inline(always)]
    pub fn new_uniform(scale: f64) -> Self {
        Self { x: scale, y: scale }
    }

    /// Restricts absolute value of scale to certain interval.
    ///
    /// # Arguments
    ///
    /// * `minimum_scale`: minimum scale along horizontal and vertical axes; should be positive.
    /// * `maximum_scale`: maximum scale along horizontal and vertical axes; should be greater than
    /// `minimum_scale`.
    ///
    /// returns: [`Scale`] - scale with absolute value restricted to given limits.
    ///
    /// # Examples
    ///
    /// ```
    /// use starry_mosaic::transform::Scale;
    ///
    /// let scale = Scale::new(0.0, -100.0);
    /// let clamped_scale = scale.clamp(0.1, 10.0);
    ///
    /// assert_eq!(clamped_scale, Scale::new(0.1, -10.0));
    /// ```
    pub fn clamp(&self, minimum_scale: f64, maximum_scale: f64) -> Self {
        assert!(minimum_scale >= 0.0);
        Self {
            x: self.x.signum() * self.x.abs().clamp(minimum_scale, maximum_scale),
            y: self.y.signum() * self.y.abs().clamp(minimum_scale, maximum_scale),
        }
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self { x: 1.0, y: 1.0 }
    }
}

impl From<f64> for Scale {
    fn from(scale: f64) -> Self {
        Self { x: scale, y: scale }
    }
}
impl From<(f64, f64)> for Scale {
    fn from(scale: (f64, f64)) -> Self {
        Self {
            x: scale.0,
            y: scale.1,
        }
    }
}

impl PartialEq for Scale {
    fn eq(&self, scale: &Self) -> bool {
        utility::approx_eq(self.x, scale.x) && utility::approx_eq(self.y, scale.y)
    }
}

impl Mul for Scale {
    type Output = Scale;
    fn mul(self, scale: Self) -> Self::Output {
        Scale {
            x: self.x * scale.x,
            y: self.y * scale.y,
        }
    }
}
impl Div for Scale {
    type Output = Scale;
    fn div(self, scale: Self) -> Self::Output {
        Scale {
            x: self.x / scale.x,
            y: self.y / scale.y,
        }
    }
}

impl Neg for Scale {
    type Output = Scale;
    fn neg(self) -> Self::Output {
        Scale {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl MulAssign for Scale {
    fn mul_assign(&mut self, scale: Self) {
        self.x *= scale.x;
        self.y *= scale.y;
    }
}
impl DivAssign for Scale {
    fn div_assign(&mut self, scale: Self) {
        self.x /= scale.x;
        self.y /= scale.y;
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts;

    use super::*;

    #[test]
    fn add_transformation() {
        let first = Transformation {
            translation: Vector::new(100.0, 100.0),
            rotation_angle: consts::FRAC_PI_6,
            scale: Scale::new(0.5, 0.75),
            shear: Vector::new(0.5, -0.5),
        };
        let second = Transformation {
            translation: Vector::new(150.0, -50.0),
            rotation_angle: consts::FRAC_PI_3,
            scale: Scale::new(1.5, 2.0),
            shear: Vector::new(-0.25, 1.0),
        };
        let sum = first + second;
        assert_eq!(
            sum,
            Transformation {
                translation: Vector::new(250.0, 50.0),
                rotation_angle: consts::FRAC_PI_2,
                scale: Scale::new(0.75, 1.5),
                shear: Vector::new(0.25, 0.5)
            }
        );
    }
    #[test]
    fn sub_transformation() {
        let first = Transformation {
            translation: Vector::new(200.0, -125.0),
            rotation_angle: consts::FRAC_PI_2,
            scale: Scale::new(1.5, 2.5),
            shear: Vector::new(1.0, 0.5),
        };
        let second = Transformation {
            translation: Vector::new(-150.0, 225.0),
            rotation_angle: consts::FRAC_PI_4,
            scale: Scale::new(2.0, 1.0),
            shear: Vector::new(0.5, 1.0),
        };
        let difference = first - second;
        assert_eq!(
            difference,
            Transformation {
                translation: Vector::new(350.0, -350.0),
                rotation_angle: consts::FRAC_PI_4,
                scale: Scale::new(0.75, 2.5),
                shear: Vector::new(0.5, -0.5)
            }
        );
    }
    #[test]
    fn neg_transformation() {
        let transformation = Transformation {
            translation: Vector::new(75.0, -85.0),
            rotation_angle: -consts::FRAC_PI_2,
            scale: Scale::default(),
            shear: Vector::new(0.3, -0.6),
        };
        assert_eq!(
            -transformation,
            Transformation {
                translation: Vector::new(-75.0, 85.0),
                rotation_angle: consts::FRAC_PI_2,
                scale: Scale::new(-1.0, -1.0),
                shear: Vector::new(-0.3, 0.6)
            }
        );
    }
    #[test]
    fn add_assign_transformation() {
        let mut transformation = Transformation {
            translation: Vector::new(0.0, 200.0),
            rotation_angle: consts::FRAC_PI_3,
            scale: Scale::new(1.5, 2.0),
            shear: Vector::new(-0.5, -0.5),
        };
        transformation += Transformation {
            translation: Vector::new(150.0, 0.0),
            rotation_angle: consts::FRAC_PI_6,
            scale: Scale::new(1.5, 2.0),
            shear: Vector::new(-0.75, 1.0),
        };
        assert_eq!(
            transformation,
            Transformation {
                translation: Vector::new(150.0, 200.0),
                rotation_angle: consts::FRAC_PI_2,
                scale: Scale::new(2.25, 4.0),
                shear: Vector::new(-1.25, 0.5)
            }
        );
    }
    #[test]
    fn sub_assign_transformation() {
        let mut transformation = Transformation {
            translation: Vector::new(-50.0, 75.0),
            rotation_angle: consts::FRAC_PI_2,
            scale: Scale::new(2.5, 2.0),
            shear: Vector::new(0.3, 0.5),
        };
        transformation -= Transformation {
            translation: Vector::new(150.0, -225.0),
            rotation_angle: consts::FRAC_PI_4,
            scale: Scale::new(2.0, 2.0),
            shear: Vector::new(0.6, -0.5),
        };
        assert_eq!(
            transformation,
            Transformation {
                translation: Vector::new(-200.0, 300.0),
                rotation_angle: consts::FRAC_PI_4,
                scale: Scale::new(1.25, 1.0),
                shear: Vector::new(-0.3, 1.0)
            }
        );
    }
    #[test]
    fn clamp_scale() {
        let scale = Scale::new(0.0, -2000.0);
        let clamped_scale = scale.clamp(0.001, 1000.0);
        assert_eq!(clamped_scale.x, 0.001);
        assert_eq!(clamped_scale.y, -1000.0);
    }
    #[test]
    fn mul_scale() {
        let first = Scale::new(0.6, 3.0);
        let second = Scale::new(7.0, 0.5);
        let sum = first * second;
        assert_eq!(sum.x, 4.2);
        assert_eq!(sum.y, 1.5);
    }
    #[test]
    fn div_scale() {
        let first = Scale::new(0.8, 4.0);
        let second = Scale::new(4.0, 2.5);
        let sum = first / second;
        assert_eq!(sum.x, 0.2);
        assert_eq!(sum.y, 1.6);
    }
    #[test]
    fn neg_scale() {
        let scale = Scale::new_uniform(-2.0);
        let negated_scale = -scale;
        assert_eq!(negated_scale.x, 2.0);
        assert_eq!(negated_scale.y, 2.0);
    }
    #[test]
    fn mul_assign_scale() {
        let mut scale = Scale::new(2.0, 0.5);
        scale *= Scale::new(3.0, 4.0);
        assert_eq!(scale.x, 6.0);
        assert_eq!(scale.y, 2.0);
    }
    #[test]
    fn div_assign_scale() {
        let mut scale = Scale::new(-3.0, 1.0);
        scale /= Scale::new(-1.0, 2.0);
        assert_eq!(scale.x, 3.0);
        assert_eq!(scale.y, 0.5);
    }
}
