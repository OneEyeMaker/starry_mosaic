use image::{Rgb, RgbImage};
use palette::{IntoColor, LinSrgb, Mix, Pixel, Shade};
use robust::Coord;
use voronoice::Voronoi;

use super::{
    coloring_method::ColoringMethod,
    mosaic::Mosaic,
    mosaic_builder::MosaicBuilder,
    mosaic_shape::MosaicShape,
    transform::{Transformation, TryToTransform},
    vector::Vector,
};

/// Represents polygonal mosaic and creates mosaic images painted with with different
/// [methods][`ColoringMethod`].
///
/// Every mosaic created by `PolygonalMosaic` is based on Delaunay triangulation, which is built
/// using key points of mosaic shape.
///
/// To create `PolygonalMosaic` instance use [MosaicBuilder][`super::mosaic_builder::MosaicBuilder`].
///
/// # See also
///
/// * [MosaicBuilder::build_polygon][`super::mosaic_builder::MosaicBuilder::build_polygon`].
///
#[derive(Clone, Debug)]
pub struct PolygonalMosaic {
    voronoi: Voronoi,
    image_size: (u32, u32),
    transformation: Transformation,
    shape: Box<dyn MosaicShape>,
}

impl PolygonalMosaic {
    pub(crate) fn new(
        voronoi: Voronoi,
        image_size: (u32, u32),
        transformation: Transformation,
        shape: Box<dyn MosaicShape>,
    ) -> Self {
        Self {
            voronoi,
            image_size,
            transformation,
            shape,
        }
    }

    fn draw_triangle<Color, Method>(
        &self,
        mosaic_image: &mut RgbImage,
        coloring_method: &Method,
        vertex_index: usize,
    ) where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        Method: ColoringMethod<Color>,
    {
        let sites = self.voronoi.sites();
        let triangulation = self.voronoi.triangulation();
        let vertex_position: Vector = (&self.voronoi.vertices()[vertex_index]).into();
        let corner_positions: [Coord<f64>; 3] = [
            (&sites[triangulation.triangles[vertex_index * 3]]).into(),
            (&sites[triangulation.triangles[vertex_index * 3 + 1]]).into(),
            (&sites[triangulation.triangles[vertex_index * 3 + 2]]).into(),
        ];
        let radius = vertex_position.distance_to(corner_positions[0].into());
        let x_min = f64::min(corner_positions[0].x, corner_positions[1].x)
            .min(corner_positions[2].x)
            .round() as u32;
        let x_max = f64::max(corner_positions[0].x, corner_positions[1].x)
            .max(corner_positions[2].x)
            .round() as u32;
        let y_min = f64::min(corner_positions[0].y, corner_positions[1].y)
            .min(corner_positions[2].y)
            .round() as u32;
        let y_max = f64::max(corner_positions[0].y, corner_positions[1].y)
            .max(corner_positions[2].y)
            .round() as u32;
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let position = Vector::new(x as f64, y as f64);
                let orientations = [
                    robust::orient2d(corner_positions[0], corner_positions[1], position.into()),
                    robust::orient2d(corner_positions[1], corner_positions[2], position.into()),
                    robust::orient2d(corner_positions[2], corner_positions[0], position.into()),
                ];
                if orientations[0] <= 0.0 && orientations[1] <= 0.0 && orientations[2] <= 0.0 {
                    let distance = position.distance_to(vertex_position);
                    let lightness = (1.0 - distance / radius).powi(2);
                    let color = coloring_method
                        .interpolate(position, vertex_position)
                        .lighten(lightness)
                        .into_color();
                    mosaic_image.put_pixel(x, y, Rgb(color.into_format().into_raw()));
                }
            }
        }
    }
}

impl Mosaic for PolygonalMosaic {
    fn draw<Color, Method>(&self, coloring_method: Method) -> RgbImage
    where
        Color: IntoColor<LinSrgb<f64>> + Mix<Scalar = f64> + Shade<Scalar = f64> + Clone,
        Method: ColoringMethod<Color>,
    {
        let mut mosaic_image = RgbImage::new(self.image_size.0, self.image_size.1);
        let vertices_count = self.voronoi.triangulation().triangles.len() / 3;
        for vertex_index in 0..vertices_count {
            self.draw_triangle(&mut mosaic_image, &coloring_method, vertex_index);
        }
        mosaic_image
    }

    fn image_size(&self) -> (u32, u32) {
        self.image_size
    }

    fn transformation(&self) -> &Transformation {
        &self.transformation
    }

    fn shape(&self) -> &Box<dyn MosaicShape> {
        &self.shape
    }
}
impl TryToTransform for PolygonalMosaic {
    fn try_to_transform(&self, transformation: &Transformation) -> Option<Self> {
        MosaicBuilder::from(self)
            .set_transformation(transformation)
            .build_polygon()
    }
}
