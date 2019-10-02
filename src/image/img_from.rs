use crate::png::Png;
use crate::image::Image;

impl From<&Png> for Image {
    fn from(png: &Png) -> Self {
        Image {
            height: png.height,
            width: png.width,
            bits_per_pixel: png.bit_depth as u32,
            pixels: png.image.clone(),
            transparency: png.transparency_index,
            next_palette: vec![],
            rgb_pal: png.palette.clone(),
        }
    }
}

