extern crate bitstream_io;

use std::fs::File;
use std::io::{BufWriter, Write};

use byteorder::WriteBytesExt;

use crate::cmdline::PalettePlacement;
use crate::convert::{PaletteEntry, LAYER2_DEFAULT};
use crate::image::{Image, ImageType, PixelFormat};
use crate::image::ImageError;

use crate::image::resample::Resample;
use crate::primitives::rectangle::Rectangle;

impl Image {
    pub fn rect(&self) -> Rectangle {
        Rectangle {
            left: 0,
            top: 0,
            width: self.width as isize,
            height: self.height as isize,
        }
    }

    pub fn resample(&mut self, format: PixelFormat) -> Result<(), ImageError> {
        let out_bits = match format {
            PixelFormat::FourBit => 4,
            PixelFormat::EightBit => 8,
        };
        if self.bits_per_pixel < out_bits {
            self.indexed_upsample(out_bits)?;
        } else if self.bits_per_pixel > 8 {
            self.down_sample(out_bits)?;
        }
        Ok(())
    }

    fn write_pal(&mut self, w: &mut BufWriter<File>, transparency: Option<u8>) -> Result<(), ImageError> {
        if self.next_palette.is_empty() {
            self.convert_palette();
        }


        for entry in &self.next_palette {
            if w.write_u16::<byteorder::LittleEndian>(entry.clone()).is_err() {
                return Err(ImageError::IOError { m: "Error writing palette info".to_string() });
            };
        }
        let palette_len = 256 - self.next_palette.len();
        for _c in 0..palette_len {
            if w.write_u16::<byteorder::LittleEndian>(0).is_err() {
                return Err(ImageError::IOError { m: "Error writing palette info".to_string() });
            };
        }
        if let Some(b) = transparency {
            if w.write_u8(b).is_err() {
                return Err(ImageError::IOError { m: "Error writing transparency info".to_string() });
            };
        }
        Ok(())
    }

    fn save_raw(&mut self, name: &str, save_palette: PalettePlacement) -> Result<(), ImageError> {
        let file = File::create(name)?;
        let mut writer = BufWriter::new(file);
        if save_palette == PalettePlacement::Prepend {
            self.write_pal(&mut writer, None)?;
        }
        if writer.write_all(&self.pixels).is_err() {
            return Err(ImageError::IOError { m: "Error writing pixel data".to_string() });
        };
        if save_palette == PalettePlacement::Append {
            self.write_pal(&mut writer, None)?;
        }
        Ok(())
    }

    fn save_pal(&mut self, name: &str, transparency: Option<u8>) -> Result<(), ImageError> {
        let file = File::create(name)?;
        let mut writer = BufWriter::new(file);
        self.write_pal(&mut writer, transparency)?;
        Ok(())
    }

    fn convert_palette(&mut self) {
        self.next_palette = vec![0; 256];
        let mut c = 0;
        for rgb in &self.rgb_pal {
            self.next_palette[c] = rgb.as_9bit();
            c += 1;
        }
    }

    fn validate_size(&self, w: usize, h: usize) -> Result<(), ImageError> {
        if self.width != w || self.height != h || self.bits_per_pixel != 8 {
            return Err(ImageError::L2Size);
        }
        Ok(())
    }

    pub fn remap_colours(&mut self, new: &[u16]) -> Result<(), ImageError> {
        if self.bits_per_pixel != 8 {
            return Err(ImageError::BitDepth { bpp: self.bits_per_pixel as u8 });
        }
        if self.next_palette.is_empty() {
            self.convert_palette();
        }
        let mut old_index = 0usize;
        let mut remap_data: Vec<u8> = vec![0; 256];
        for colour in &self.next_palette {
            let alt_color = colour ^ 0x0100;
            if let Some(new_index) = new.iter().position(|f| f == colour || f == &alt_color) {
                remap_data[old_index] = new_index as u8;
            } else {
                return Err(ImageError::PaletteRemap);
            }
            old_index += 1;
        }
        let mut remapped: Vec<u8> = vec![];
        for pixel in &self.pixels {
            remapped.push(remap_data[pixel.clone() as usize]);
        }
        self.pixels.clear();
        self.pixels.append(&mut remapped);
        Ok(())
    }


    pub fn save(&mut self, image_type: ImageType, name: &str) -> Result<(), ImageError> {
        match image_type {
            ImageType::Raw => self.save_raw(name, PalettePlacement::Append),
            ImageType::Nxi => {
                self.validate_size(256, 192)?;
                self.save_raw(name, PalettePlacement::Prepend)
            }
            ImageType::Asm => unimplemented!(),
            ImageType::Pal => self.save_pal(name, None),
            ImageType::Npl => self.save_pal(name, Some(self.transparency)),
            ImageType::Sl2 => {
                self.validate_size(256, 192)?;
                self.remap_colours(LAYER2_DEFAULT)?;
                self.save_raw(name, PalettePlacement::Omit)
            }
            ImageType::Slr => {
                self.validate_size(128, 96)?;
                self.remap_colours(LAYER2_DEFAULT)?;
                self.save_raw(name, PalettePlacement::Omit)
            }
        }
    }
}