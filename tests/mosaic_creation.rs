use std::f64::consts;

use palette::{IntoColor, LinSrgb, Mix, Shade};
use rstest::rstest;
use rstest_reuse::{self, *};
use starry_mosaic::{
    coloring_method::*, mosaic_shape::*, transform::Scale, Mosaic, MosaicBuilder, Vector,
};

#[template]
#[rstest]
#[case(RegularPolygon::new(5), "regular_polygon_5")]
#[case(RegularPolygon::new(6), "regular_polygon_6")]
#[case(RegularPolygon::new(7), "regular_polygon_7")]
#[case(RegularPolygon::new(8), "regular_polygon_8")]
#[case(RegularPolygon::new(12), "regular_polygon_12")]
#[case(RegularPolygon::new(13), "regular_polygon_13")]
#[case(RegularPolygon::new(15), "regular_polygon_15")]
#[case(RegularPolygon::new(16), "regular_polygon_16")]
#[case(RegularPolygon::new(23), "regular_polygon_23")]
#[case(RegularPolygon::new(24), "regular_polygon_24")]
#[case(PolygonalStar::new(5), "polygonal_star_5")]
#[case(PolygonalStar::new(6), "polygonal_star_6")]
#[case(PolygonalStar::new(7), "polygonal_star_7")]
#[case(PolygonalStar::new(8), "polygonal_star_8")]
#[case(PolygonalStar::new(12), "polygonal_star_12")]
#[case(PolygonalStar::new(13), "polygonal_star_13")]
#[case(PolygonalStar::new(15), "polygonal_star_15")]
#[case(PolygonalStar::new(16), "polygonal_star_16")]
#[case(PolygonalStar::new(23), "polygonal_star_23")]
#[case(PolygonalStar::new(24), "polygonal_star_24")]
#[case(Grid::new(3, 3), "grid_3_3")]
#[case(Grid::new(4, 4), "grid_4_4")]
#[case(Grid::new(5, 5), "grid_5_5")]
#[case(Grid::new(8, 8), "grid_8_8")]
#[case(Grid::new(12, 12), "grid_12_12")]
#[case(Grid::new(15, 15), "grid_15_15")]
#[case(Grid::new(16, 16), "grid_16_16")]
#[case(Grid::new(24, 24), "grid_24_24")]
#[case(Grid::new(6, 9), "grid_6_9")]
#[case(Grid::new(11, 7), "grid_11_7")]
#[case(Grid::new(13, 23), "grid_13_23")]
#[case(Grid::new(21, 14), "grid_21_14")]
fn mosaic_creation_test<Shape>(#[case] shape: Shape, #[case] name: &str)
where
    Shape: 'static + MosaicShape,
{
}

fn create_gradient() -> Vec<(f64, LinSrgb<f64>)> {
    vec![
        (0.0, LinSrgb::new(0.0f64, 0.0, 1.0)),
        (1.0 / 3.0, LinSrgb::new(0.0f64, 0.75, 1.0)),
        (2.0 / 3.0, LinSrgb::new(1.0f64, 1.0, 0.0)),
        (1.0, LinSrgb::new(1.0f64, 0.0, 0.25)),
    ]
}
fn create_symmetric_gradient() -> Vec<(f64, LinSrgb<f64>)> {
    vec![
        (0.0, LinSrgb::new(0.0f64, 0.0, 1.0)),
        (1.0 / 6.0, LinSrgb::new(0.0f64, 0.75, 1.0)),
        (1.0 / 3.0, LinSrgb::new(1.0f64, 1.0, 0.0)),
        (0.5, LinSrgb::new(1.0f64, 0.0, 0.25)),
        (2.0 / 3.0, LinSrgb::new(1.0f64, 1.0, 0.0)),
        (5.0 / 6.0, LinSrgb::new(0.0f64, 0.75, 1.0)),
        (1.0, LinSrgb::new(0.0f64, 0.0, 1.0)),
    ]
}
fn create_linear_gradient() -> LinearGradient<LinSrgb<f64>> {
    LinearGradient::new(
        create_gradient(),
        Vector::new(320.0, 560.0),
        Vector::new(1280.0, 1040.0),
        0.5,
    )
}
fn create_radial_gradient() -> RadialGradient<LinSrgb<f64>> {
    RadialGradient::new(
        create_gradient(),
        Vector::new(1200.0, 640.0),
        60.0,
        Vector::new(800.0, 800.0),
        600.0,
        0.5,
    )
}
fn create_conic_gradient() -> ConicGradient<LinSrgb<f64>> {
    ConicGradient::new(
        create_symmetric_gradient(),
        Vector::new(800.0, 800.0),
        -consts::FRAC_PI_3,
        0.5,
    )
}

mod starry_mosaic_tests {
    use super::*;

    fn create_starry_mosaic<Color, Method, Shape>(
        shape: Shape,
        coloring_method: Method,
        group: &str,
        name: &str,
    ) where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        Method: ColoringMethod<Color>,
        Shape: 'static + MosaicShape,
    {
        let image_size = (1600, 1600);
        let center = Vector::new(800.0, 800.0);
        let rotation_angle = consts::PI;
        let scale = 0.7;

        let mosaic = MosaicBuilder::default()
            .set_shape(shape)
            .set_image_size(image_size.0, image_size.1)
            .set_center(center.clone())
            .set_rotation_angle(rotation_angle)
            .set_uniform_scale(scale)
            .build_star();
        assert!(mosaic.is_some());

        let mosaic = mosaic.unwrap();
        assert_eq!(mosaic.image_size(), image_size);
        assert_eq!(mosaic.transformation().translation, center);
        assert_eq!(mosaic.transformation().rotation_angle, rotation_angle);
        assert_eq!(mosaic.transformation().scale, Scale::new_uniform(scale));

        let mosaic_image = mosaic.draw(coloring_method);
        let save_result = mosaic_image.save(format!("images/starry_mosaic/{}/{}.png", group, name));
        assert!(save_result.is_ok());
    }
    #[apply(mosaic_creation_test)]
    fn single_colored_mosaic<Shape>(shape: Shape, name: &str)
    where
        Shape: 'static + MosaicShape,
    {
        let color = LinSrgb::new(0.0f64, 0.0, 1.0);
        create_starry_mosaic(shape, color, "single_colored", name);
    }
    #[apply(mosaic_creation_test)]
    fn linear_gradient_mosaic<Shape>(shape: Shape, name: &str)
    where
        Shape: 'static + MosaicShape,
    {
        let linear_gradient = create_linear_gradient();
        create_starry_mosaic(shape, linear_gradient, "linear_gradient", name);
    }
    #[apply(mosaic_creation_test)]
    fn radial_gradient_mosaic<Shape>(shape: Shape, name: &str)
    where
        Shape: 'static + MosaicShape,
    {
        let radial_gradient = create_radial_gradient();
        create_starry_mosaic(shape, radial_gradient, "radial_gradient", name);
    }
    #[apply(mosaic_creation_test)]
    fn conic_gradient_mosaic<Shape>(shape: Shape, name: &str)
    where
        Shape: 'static + MosaicShape,
    {
        let conic_gradient = create_conic_gradient();
        create_starry_mosaic(shape, conic_gradient, "conic_gradient", name);
    }
}
mod polygonal_mosaic_tests {
    use super::*;

    fn create_polygonal_mosaic<Color, Method, Shape: 'static>(
        shape: Shape,
        coloring_method: Method,
        group: &str,
        name: &str,
    ) where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        Method: ColoringMethod<Color>,
        Shape: 'static + MosaicShape,
    {
        let image_size = (1600, 1600);
        let center = Vector::new(800.0, 800.0);
        let rotation_angle = consts::PI;
        let scale = 0.7;

        let mosaic = MosaicBuilder::default()
            .set_shape(shape)
            .set_image_size(image_size.0, image_size.1)
            .set_center(center.clone())
            .set_rotation_angle(rotation_angle)
            .set_uniform_scale(scale)
            .build_polygon();
        assert!(mosaic.is_some());

        let mosaic = mosaic.unwrap();
        assert_eq!(mosaic.image_size(), image_size);
        assert_eq!(mosaic.transformation().translation, center);
        assert_eq!(mosaic.transformation().rotation_angle, rotation_angle);
        assert_eq!(mosaic.transformation().scale, Scale::new_uniform(scale));

        let mosaic_image = mosaic.draw(coloring_method);
        let save_result =
            mosaic_image.save(format!("images/polygonal_mosaic/{}/{}.png", group, name));
        assert!(save_result.is_ok());
    }
    #[apply(mosaic_creation_test)]
    fn single_colored_mosaic<Shape>(shape: Shape, name: &str)
    where
        Shape: 'static + MosaicShape,
    {
        let color = LinSrgb::new(0.0f64, 0.0, 1.0);
        create_polygonal_mosaic(shape, color, "single_colored", name);
    }
    #[apply(mosaic_creation_test)]
    fn linear_gradient_mosaic<Shape>(shape: Shape, name: &str)
    where
        Shape: 'static + MosaicShape,
    {
        let linear_gradient = create_linear_gradient();
        create_polygonal_mosaic(shape, linear_gradient, "linear_gradient", name);
    }
    #[apply(mosaic_creation_test)]
    fn radial_gradient_mosaic<Shape>(shape: Shape, name: &str)
    where
        Shape: 'static + MosaicShape,
    {
        let radial_gradient = create_radial_gradient();
        create_polygonal_mosaic(shape, radial_gradient, "radial_gradient", name);
    }
    #[apply(mosaic_creation_test)]
    fn conic_gradient_mosaic<Shape>(shape: Shape, name: &str)
    where
        Shape: 'static + MosaicShape,
    {
        let conic_gradient = create_conic_gradient();
        create_polygonal_mosaic(shape, conic_gradient, "conic_gradient", name);
    }
}
