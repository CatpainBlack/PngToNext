use crate::image::{ImageType, PixelFormat};
use crate::primitives::rectangle::Rectangle;

pub mod parser;

#[derive(Clone, PartialEq)]
pub enum PalettePlacement {
    Omit,
    Prepend,
    Append,
    //Separate,
}

pub struct Options {
    // Files
    pub png_file_name: String,
    pub out_file_name: String,

    //
    pub output_type: ImageType,
    pub pal_priority: bool,
    pub crop_to: Option<Rectangle>,

    // Palette
    pal_placement: PalettePlacement,

    pixel_format: PixelFormat,

    // Internal
    crop: String,

    // Behaviour
    pub verbose: bool,
}

