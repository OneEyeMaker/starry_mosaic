use std::f64::consts;

use super::Vector;

pub fn set_up_polygon_points(
    corners_count: u32,
    radius: f64,
    center_point: Vector,
    rotation_angle: f64,
) -> Vec<Vector> {
    let mut points = Vec::new();
    for index in 0..corners_count {
        let angle = rotation_angle
            + consts::PI / corners_count as f64 * (2 * index + 1 - corners_count % 2) as f64
            - consts::FRAC_PI_2;
        points.push(Vector::new(
            center_point.x + radius * angle.cos(),
            center_point.y + radius * angle.sin(),
        ));
    }
    points
}
