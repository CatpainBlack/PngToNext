use crate::cmdline::Options;
use crate::convert::Process;
use crate::errors::CmdError;
use crate::image::{BlockSet, Image, ImageType};
use crate::image::PixelFormat;
use crate::png::Png;
use crate::primitives::rectangle::Rectangle;

pub struct ImageConverter<'a> {
    source: &'a Png,
    image_type: ImageType,
}

impl<'a> ImageConverter<'a> {
    pub fn new(source: &Png) -> ImageConverter {
        ImageConverter {
            source,
            image_type: ImageType::Raw,
        }
    }

    pub fn output_format(&mut self, image_type: ImageType) -> &ImageConverter {
        self.image_type = image_type;
        self
    }

    pub fn save_as(&self, file_name: &str) -> Result<(), CmdError> {
        let mut img = Image::from(self.source);
        img.save(&self.image_type, file_name)?;
        Ok(())
    }
}