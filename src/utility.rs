use float_cmp::ApproxEq;

pub const EPSILON: f64 = f32::EPSILON as f64;
const ONE_OVER_EPSILON: f64 = 1.0 / EPSILON;

#[inline(always)]
pub fn approx_eq(left: f64, right: f64) -> bool {
    left.approx_eq(right, (EPSILON, 4))
}

#[inline(always)]
pub fn round_to_epsilon(number: f64) -> f64 {
    (number * ONE_OVER_EPSILON).round() * EPSILON
}
