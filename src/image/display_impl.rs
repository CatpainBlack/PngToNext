use crate::image::ImageType;
use std::fmt::{Display, Formatter, Error};


impl Display for ImageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            ImageType::Raw => write!(f, "Raw Data"),
            ImageType::Nxi => write!(f, "NXI"),
            ImageType::Asm => write!(f, "Assembler Source"),
            ImageType::Pal => write!(f, "Pal"),
            ImageType::Npl => write!(f, "Npl"),
            ImageType::Sl2 => write!(f, "Sl2"),
            ImageType::Slr => write!(f, "slr"),
            ImageType::Spr => write!(f, "Sprite data")
        }
    }
}