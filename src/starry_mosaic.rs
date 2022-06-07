use image::{Rgb, RgbImage};
use palette::{IntoColor, LinSrgb, Mix, Pixel, Shade};
use voronoice::Voronoi;

use super::{coloring_method::ColoringMethod, mosaic::Mosaic, vector::Vector};

#[derive(Clone, Debug)]
pub struct StarryMosaic {
    voronoi: Voronoi,
    image_size: (u32, u32),
}

impl StarryMosaic {
    pub(crate) fn new(voronoi: Voronoi, image_size: (u32, u32)) -> Self {
        Self {
            voronoi,
            image_size,
        }
    }
    fn calculate_maximum_cell_distances(&self) -> Vec<f64> {
        let mut maximum_cell_distances = vec![0.0f64; self.voronoi.cells().len()];
        self.voronoi.iter_cells().for_each(|cell| {
            let site = cell.site();
            let site_position: Vector = cell.site_position().into();
            cell.iter_vertices().for_each(|vertex| {
                let distance = (&site_position - &vertex.into()).length();
                if distance > maximum_cell_distances[site] {
                    maximum_cell_distances[site] = distance;
                }
            });
        });
        maximum_cell_distances
    }
    fn find_closest_site(&self, site: usize, vector: &Vector) -> usize {
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
            let site = self.find_closest_site(current_site, &position);
            if site == 0 || current_site != site {
                current_site = site;
                current_site_position = (&self.voronoi.sites()[current_site]).into();
            }
            let distance = (&position - &current_site_position).length();
            let lightness = (1.0 - distance / maximum_cell_distances[current_site]).powi(2);
            let color = coloring_method
                .interpolate(&position, &current_site_position)
                .lighten(lightness)
                .into_color();
            *pixel = Rgb(color.into_format().into_raw())
        }
        mosaic_image
    }
}