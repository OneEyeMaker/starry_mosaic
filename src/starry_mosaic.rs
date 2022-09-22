use image::{Rgb, RgbImage};
use palette::{IntoColor, LinSrgb, Mix, Pixel, Shade};
use voronoice::Voronoi;

use super::{
    coloring_method::ColoringMethod, mosaic::Mosaic, mosaic_shape::MosaicShape, vector::Vector,
};

/// Represents starry mosaic and creates mosaic images painted with with different [methods][`ColoringMethod`].
///
/// Every mosaic created by `StarryMosaic` is based on Voronoi diagram, which is built
/// using key points of mosaic shape.
///
/// To create `StarryMosaic` instance use [MosaicBuilder][`super::mosaic_builder::MosaicBuilder`].
///
/// # See also
///
/// * [MosaicBuilder::build_star][`super::mosaic_builder::MosaicBuilder::build_star`].
///
#[derive(Clone, Debug)]
pub struct StarryMosaic {
    voronoi: Voronoi,
    image_size: (u32, u32),
    center: Vector,
    rotation_angle: f64,
    scale: f64,
    shape: Box<dyn MosaicShape>,
}

impl StarryMosaic {
    pub(crate) fn new(
        voronoi: Voronoi,
        image_size: (u32, u32),
        center: Vector,
        rotation_angle: f64,
        scale: f64,
        shape: Box<dyn MosaicShape>,
    ) -> Self {
        Self {
            voronoi,
            image_size,
            center,
            rotation_angle,
            scale,
            shape,
        }
    }

    fn calculate_maximum_cell_distances(&self) -> Vec<f64> {
        let mut maximum_cell_distances = vec![0.0f64; self.voronoi.cells().len()];
        self.voronoi.iter_cells().for_each(|cell| {
            let site = cell.site();
            let site_position: Vector = cell.site_position().into();
            cell.iter_vertices().for_each(|vertex| {
                let distance = site_position.distance_to(vertex.into());
                if distance > maximum_cell_distances[site] {
                    maximum_cell_distances[site] = distance;
                }
            });
        });
        maximum_cell_distances
    }

    fn find_closest_site(&self, site: usize, vector: Vector) -> usize {
        self.voronoi
            .cell(site)
            .iter_path(vector.into())
            .last()
            .unwrap_or(site)
    }
}

impl Mosaic for StarryMosaic {
    fn draw<Color, Method>(&self, coloring_method: Method) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        Method: ColoringMethod<Color>,
    {
        let maximum_cell_distances = self.calculate_maximum_cell_distances();
        let mut mosaic_image = RgbImage::new(self.image_size.0, self.image_size.1);
        let mut current_site = 0;
        let mut current_site_position = Vector::default();
        for (x, y, pixel) in mosaic_image.enumerate_pixels_mut() {
            let position = Vector::new(x as f64, y as f64);
            let site = self.find_closest_site(current_site, position);
            if site == 0 || current_site != site {
                current_site = site;
                current_site_position = (&self.voronoi.sites()[current_site]).into();
            }
            let distance = position.distance_to(current_site_position);
            let lightness = (1.0 - distance / maximum_cell_distances[current_site]).powi(2);
            let color = coloring_method
                .interpolate(position, current_site_position)
                .lighten(lightness)
                .into_color();
            *pixel = Rgb(color.into_format().into_raw());
        }
        mosaic_image
    }

    fn image_size(&self) -> (u32, u32) {
        self.image_size
    }

    fn center(&self) -> Vector {
        self.center
    }

    fn rotation_angle(&self) -> f64 {
        self.rotation_angle
    }

    fn scale(&self) -> f64 {
        self.scale
    }

    fn shape(&self) -> &Box<dyn MosaicShape> {
        &self.shape
    }
}
