use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::{utility, vector::Vector};

#[derive(Clone, Debug, Default)]
pub struct Transformation {
    pub translation: Vector,
    pub scale: Scale,
    pub shear: Vector,
    pub rotation_angle: f64,
}

impl Transformation {
    pub fn from_translation<VectorLike>(translation: VectorLike) -> Self
    where
        VectorLike: Into<Vector>,
    {
        let mut transformation = Transformation::default();
        transformation.translation = translation.into();
        transformation
    }

    pub fn from_rotation(rotation_angle: f64) -> Self {
        let mut transformation = Transformation::default();
        transformation.rotation_angle = rotation_angle;
        transformation
    }

    pub fn from_scale<ScaleLike>(scale: ScaleLike) -> Self
    where
        ScaleLike: Into<Scale>,
    {
        let mut transformation = Transformation::default();
        transformation.scale = scale.into();
        transformation
    }

    pub fn from_shear<VectorLike>(shear: VectorLike) -> Self
    where
        VectorLike: Into<Vector>,
    {
        let mut transformation = Transformation::default();
        transformation.shear = shear.into();
        transformation
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

#[derive(Clone, Copy, Debug)]
pub struct Scale {
    pub x: f64,
    pub y: f64,
}

impl Scale {
    #[inline(always)]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn new_uniform(scale: f64) -> Self {
        Self { x: scale, y: scale }
    }

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
    use super::*;

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
