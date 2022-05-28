use float_cmp::ApproxEq;

const EPSILON: f64 = f32::EPSILON as f64;

#[inline(always)]
pub fn approx_eq(left: f64, right: f64) -> bool {
    left.approx_eq(right, (EPSILON, 4))
}
