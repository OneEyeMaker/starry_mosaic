use std::f64::consts;

use palette::{IntoColor, LinSrgb, Mix, Shade};
use rstest::rstest;
use rstest_reuse::{self, *};
use starry_mosaic::{
    coloring_method::{ColoringMethod, ConicGradient, LinearGradient, RadialGradient},
    Mosaic, MosaicBuilder, Vector,
};

#[template]
#[rstest]
#[case(3)]
#[case(4)]
#[case(5)]
#[case(6)]
#[case(7)]
#[case(8)]
#[case(9)]
#[case(10)]
#[case(11)]
#[case(12)]
#[case(13)]
#[case(14)]
#[case(15)]
#[case(16)]
#[case(17)]
#[case(18)]
#[case(19)]
#[case(20)]
#[case(21)]
#[case(22)]
#[case(23)]
#[case(24)]
fn mosaic_creation_test(#[case] corners_count: u32) {}

fn create_gradient() -> Vec<(f64, LinSrgb<f64>)> {
    vec![
        (0.1, LinSrgb::new(0.0f64, 0.0, 1.0)),
        (0.4, LinSrgb::new(0.0f64, 0.75, 1.0)),
        (0.7, LinSrgb::new(0.1f64, 1.0, 0.0)),
        (1.0, LinSrgb::new(1.0f64, 0.0, 0.25)),
    ]
}
fn create_linear_gradient() -> LinearGradient<LinSrgb<f64>> {
    LinearGradient::new(
        create_gradient(),
        Vector::new(800.0, 360.0),
        Vector::new(1600.0, 720.0),
        0.5,
    )
}
fn create_radial_gradient() -> RadialGradient<LinSrgb<f64>> {
    RadialGradient::new(
        create_gradient(),
        Vector::new(1440.0, 720.0),
        50.0,
        Vector::new(1280.0, 540.0),
        400.0,
        0.5,
    )
}
fn create_conic_gradient() -> ConicGradient<LinSrgb<f64>> {
    let gradient = vec![
        (0.0, LinSrgb::new(0.0f64, 0.0, 1.0)),
        (0.25, LinSrgb::new(0.0f64, 0.75, 1.0)),
        (0.5, LinSrgb::new(0.1f64, 1.0, 0.0)),
        (0.75, LinSrgb::new(1.0f64, 0.0, 0.25)),
        (1.0, LinSrgb::new(0.0f64, 0.0, 1.0)),
    ];
    ConicGradient::new(
        gradient,
        Vector::new(1280.0, 540.0),
        -consts::FRAC_PI_3,
        0.5,
    )
}

mod starry_mosaic_tests {
    use super::*;

    fn create_starry_mosaic<Color, Method>(
        corners_count: u32,
        coloring_method: Method,
        mosaic_name: &str,
    ) where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        Method: ColoringMethod<Color>,
    {
        let image_size = (1920, 1080);
        let center = Vector::new(1280.0, 540.0);
        let rotation_angle = consts::PI;
        let scale = 0.7;

        let mosaic = MosaicBuilder::default()
            .set_regular_polygon_shape(corners_count)
            .set_image_size(image_size.0, image_size.1)
            .set_center(center.clone())
            .set_rotation_angle(rotation_angle)
            .set_scale(scale)
            .build_star();
        assert!(mosaic.is_some());

        let mosaic = mosaic.unwrap();
        assert_eq!(mosaic.image_size(), image_size);
        assert_eq!(mosaic.center(), center);
        assert_eq!(mosaic.rotation_angle(), rotation_angle);
        assert_eq!(mosaic.scale(), scale);

        let mosaic_image = mosaic.draw(coloring_method);
        let save_result = mosaic_image.save(format!(
            "images/starry_mosaic/{}/{}.png",
            mosaic_name, corners_count
        ));
        assert!(save_result.is_ok());
    }
    #[apply(mosaic_creation_test)]
    fn single_colored_mosaic(corners_count: u32) {
        let color = LinSrgb::new(0.0f64, 0.0, 1.0);
        create_starry_mosaic(corners_count, color, "single_colored");
    }
    #[apply(mosaic_creation_test)]
    fn linear_gradient_mosaic(corners_count: u32) {
        let linear_gradient = create_linear_gradient();
        create_starry_mosaic(corners_count, linear_gradient, "linear_gradient");
    }
    #[apply(mosaic_creation_test)]
    fn radial_gradient_mosaic(corners_count: u32) {
        let radial_gradient = create_radial_gradient();
        create_starry_mosaic(corners_count, radial_gradient, "radial_gradient");
    }
    #[apply(mosaic_creation_test)]
    fn conic_gradient_mosaic(corners_count: u32) {
        let conic_gradient = create_conic_gradient();
        create_starry_mosaic(corners_count, conic_gradient, "conic_gradient");
    }
}
mod polygonal_mosaic_tests {
    use super::*;

    fn create_polygonal_mosaic<Color, Method>(
        corners_count: u32,
        coloring_method: Method,
        mosaic_name: &str,
    ) where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        Method: ColoringMethod<Color>,
    {
        let image_size = (1920, 1080);
        let center = Vector::new(1280.0, 540.0);
        let rotation_angle = consts::PI;
        let scale = 0.7;

        let mosaic = MosaicBuilder::default()
            .set_regular_polygon_shape(corners_count)
            .set_image_size(image_size.0, image_size.1)
            .set_center(center.clone())
            .set_rotation_angle(rotation_angle)
            .set_scale(scale)
            .build_polygon();
        assert!(mosaic.is_some());

        let mosaic = mosaic.unwrap();
        assert_eq!(mosaic.image_size(), image_size);
        assert_eq!(mosaic.center(), center);
        assert_eq!(mosaic.rotation_angle(), rotation_angle);
        assert_eq!(mosaic.scale(), scale);

        let mosaic_image = mosaic.draw(coloring_method);
        let save_result = mosaic_image.save(format!(
            "images/polygonal_mosaic/{}/{}.png",
            mosaic_name, corners_count
        ));
        assert!(save_result.is_ok());
    }
    #[apply(mosaic_creation_test)]
    fn single_colored_mosaic(corners_count: u32) {
        let color = LinSrgb::new(0.0f64, 0.0, 1.0);
        create_polygonal_mosaic(corners_count, color, "single_colored");
    }
    #[apply(mosaic_creation_test)]
    fn linear_gradient_mosaic(corners_count: u32) {
        let linear_gradient = create_linear_gradient();
        create_polygonal_mosaic(corners_count, linear_gradient, "linear_gradient");
    }
    #[apply(mosaic_creation_test)]
    fn radial_gradient_mosaic(corners_count: u32) {
        let radial_gradient = create_radial_gradient();
        create_polygonal_mosaic(corners_count, radial_gradient, "radial_gradient");
    }
    #[apply(mosaic_creation_test)]
    fn conic_gradient_mosaic(corners_count: u32) {
        let conic_gradient = create_conic_gradient();
        create_polygonal_mosaic(corners_count, conic_gradient, "conic_gradient");
    }
}
