//! A library for generating colorful mosaic images with various geometrical patterns.
//!
//! # How to create mosaic image
//!
//! To create mosaic simply:
//!     - set size of resulting image;
//!     - choose shape which mosaic is based on,
//!     - set its position, rotation and scale
//!     - and finally select type of mosaic.
//!
//! Then this mosaic can be painted with any color or gradient.
//!
//! In code whole process looks like this:
//!
//! ```rust
//! use palette::LinSrgb;
//! use starry_mosaic::{Mosaic, MosaicBuilder, Vector};
//!
//! let mosaic = MosaicBuilder::default()
//!     .set_image_size(1920, 1080)
//!     .set_regular_polygon_shape(12)
//!     .set_center(Vector::new(1280.0, 540.0))
//!     .set_scale(0.6)
//!     .build_star()
//!     .unwrap();
//!
//! let mosaic_image = mosaic.draw(LinSrgb::new(0.0f64, 0.25, 1.0));
//!
//! let save_result = mosaic_image.save("target/mosaic.image.png");
//! assert!(save_result.is_ok());
//! ```

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
