use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use image::RgbImage;
use palette::LinSrgb;
use starry_mosaic::{
    coloring_method::RadialGradient, mosaic_shape::*, Mosaic, MosaicBuilder, Vector,
};

fn create_image<Shape>(shape: Shape) -> RgbImage
where
    Shape: 'static + MosaicShape,
{
    let gradient = vec![
        (0.125, LinSrgb::new(1.0f64, 0.0, 0.0)),
        (0.275, LinSrgb::new(1.0f64, 0.5, 0.0)),
        (0.425, LinSrgb::new(1.0f64, 1.0, 0.0)),
        (0.575, LinSrgb::new(0.0f64, 1.0, 0.0)),
        (0.725, LinSrgb::new(0.0f64, 0.75, 1.0)),
        (0.875, LinSrgb::new(0.0f64, 0.0, 1.0)),
    ];
    let mosaic = MosaicBuilder::default()
        .set_shape(shape)
        .set_image_size(1600, 1600)
        .set_center(Vector::new(800.0, 800.0))
        .set_uniform_scale(0.75)
        .build_star()
        .unwrap();
    mosaic.draw(RadialGradient::new(
        gradient,
        Vector::new(1080.0, 640.0),
        40.0,
        Vector::new(800.0, 800.0),
        720.0,
        0.25,
    ))
}

fn regular_polygon_benchmark(instance: &mut Criterion) {
    let mut group = instance.benchmark_group("regular_polygon");
    group
        .sample_size(40)
        .measurement_time(Duration::from_secs(30));
    for corners_count in 5..=16 {
        group.bench_with_input(
            BenchmarkId::from_parameter(corners_count),
            &corners_count,
            |bencher, &corners_count| {
                bencher.iter(|| create_image(RegularPolygon::new(corners_count)));
            },
        );
    }
    group.finish();
}
fn polygonal_star_benchmark(instance: &mut Criterion) {
    let mut group = instance.benchmark_group("polygonal_star");
    group
        .sample_size(40)
        .measurement_time(Duration::from_secs(40));
    for corners_count in 5..=16 {
        group.bench_with_input(
            BenchmarkId::from_parameter(corners_count),
            &corners_count,
            |bencher, &corners_count| {
                bencher.iter(|| create_image(PolygonalStar::new(corners_count)));
            },
        );
    }
    group.finish();
}
fn tilted_grid_benchmark(instance: &mut Criterion) {
    let mut group = instance.benchmark_group("tilted_grid");
    group
        .sample_size(40)
        .measurement_time(Duration::from_secs(30));
    for cells_count in 5..=16 {
        group.bench_with_input(
            BenchmarkId::from_parameter(cells_count),
            &cells_count,
            |bencher, &cells_count| {
                bencher.iter(|| create_image(Grid::new(cells_count, cells_count)));
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    regular_polygon_benchmark,
    polygonal_star_benchmark,
    tilted_grid_benchmark
);
criterion_main!(benches);
