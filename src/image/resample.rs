use crate::image::Image;
use crate::image::ImageError;
use std::io::Cursor;
use bitstream_io::{BitReader, LittleEndian, BitWriter};
use rgb::FromSlice;
use crate::convert::PaletteEntry;
use std::collections::HashSet;

pub trait Resample {
    fn indexed_upsample(&mut self, bpp: u32) -> Result<(), ImageError>;
    fn down_sample(&mut self, bpp: u32) -> Result<(), ImageError>;
}

impl Resample for Image {
    fn indexed_upsample(&mut self, bpp: u32) -> Result<(), ImageError> {
        let mut converted: Vec<u8> = vec![];
        let pixels = Cursor::new(&self.pixels);
        let mut br = BitReader::endian(pixels, LittleEndian);
        let mut bw = BitWriter::endian(&mut converted, LittleEndian);
        for _row in 0..self.height {
            for _col in 0..self.width {
                let pix = br.read::<u8>(self.bits_per_pixel);
                if pix.is_err() {
                    return Err(ImageError::Resample);
                }
                if bw.write::<u8>(bpp, pix.unwrap()).is_err() {
                    return Err(ImageError::Resample);
                };
            }
        }
        self.pixels = converted.clone();
        self.bits_per_pixel = bpp;
        Ok(())
    }

    fn down_sample(&mut self, bpp: u32) -> Result<(), ImageError> {
        let pixels = self.pixels.as_rgb().to_vec();
        let mut pal: HashSet<u16> = HashSet::new();
        let mut reduced: Vec<u8> = vec![];
        for pixel in &pixels {
            let rgb = pixel.as_9bit();
            let index = match pal.get(&rgb) {
                None => {
                    pal.insert(rgb);
                    pal.len() - 1
                }
                Some(i) => i.clone() as usize,
            };
            reduced.push(index as u8);
        }
        self.next_palette = pal.iter().map(|f| f.clone()).collect();
        self.pixels = reduced;
        self.bits_per_pixel = bpp;
        Ok(())
    }
}