use crate::image::ImageType;
use crate::image::rectangle::Rectangle;

pub mod parser;

pub struct Options {
    // Files
    pub png_file_name: String,
    pub out_file_name: String,

    //
    pub output_type: ImageType,
    pub pal_priority: bool,
    pub bits: u8,
    pub crop_to: Option<Rectangle>,

    // Internal
    crop: String,

    // Behaviour
    pub verbose: bool,
}

