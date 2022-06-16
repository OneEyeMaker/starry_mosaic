mod utility;

mod vector;
pub use self::vector::Vector;

mod segment;
pub use self::segment::Segment;

pub mod coloring_method;

pub mod mosaic_shape;

mod mosaic;
pub use self::mosaic::Mosaic;
#[cfg(feature = "mosaic_with_preset_coloring")]
pub use self::mosaic::MosaicWithPresetColoring;

mod mosaic_builder;
pub use self::mosaic_builder::MosaicBuilder;

mod starry_mosaic;
pub use self::starry_mosaic::StarryMosaic;
