use rgb::RGB8;

extern crate custom_error;

use custom_error::custom_error;
use std::fmt::{Display, Formatter, Error};

mod img_from;
pub mod rectangle;

#[derive(Clone)]
pub enum ImageType {
    Raw,
    Nxi,
    Asm,
    Pal,
    Npl,
    Sl2,
}

pub enum PixelFormat {
    FourBit,
    EightBit,
}

pub struct Image {
    pub height: usize,
    pub width: usize,
    pub bits_per_pixel: u32,
    pub pixels: Vec<u8>,
    pub transparency: u8,
    rgb_pal: Vec<RGB8>,
}

custom_error! {pub ImageError
    BitDepth{bpp:u8}="Unsupported bit depth {bpp}",
    Resample="Could not resample pixel data",
    IOError{m:String}="IO Error {m}"
}

impl std::convert::From<std::io::Error> for ImageError {
    fn from(e: std::io::Error) -> Self {
        ImageError::IOError { m: e.to_string() }
    }
}

impl Display for ImageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            ImageType::Raw => write!(f, "Raw Data"),
            ImageType::Nxi => write!(f, "NXI"),
            ImageType::Asm => write!(f, "Assembler Source"),
            ImageType::Pal => write!(f, "Pal"),
            ImageType::Npl => write!(f, "Npl"),
            ImageType::Sl2 => write!(f, "Sl2"),
        }
    }
}