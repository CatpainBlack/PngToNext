extern crate bitstream_io;

use std::fs::File;
use std::io::{BufWriter, Cursor, Write};

use byteorder::WriteBytesExt;

use crate::cmdline::PalettePlacement;
use crate::convert::PaletteEntry;
use crate::image::{Image, ImageType, PixelFormat};
use crate::image::ImageError;

use self::bitstream_io::{BitReader, BitWriter, LittleEndian};


impl Image {
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

    pub fn resample(&mut self, format: PixelFormat) -> Result<(), ImageError> {
        if self.bits_per_pixel > 8 {
            return Err(ImageError::BitDepth { bpp: self.bits_per_pixel as u8 });
        }
        let bits = match format {
            PixelFormat::FourBit => 4,
            PixelFormat::EightBit => 8,
        };
        if bits > self.bits_per_pixel {
            self.indexed_upsample(bits)?
        }
        Ok(())
    }

    fn write_pal(&mut self, w: &mut BufWriter<File>, transparency: Option<u8>) -> Result<(), ImageError> {
        let count = 256 - self.rgb_pal.len();
        for rgb in &self.rgb_pal {
            if w.write_u16::<byteorder::LittleEndian>(rgb.as_9bit()).is_err() {
                return Err(ImageError::IOError { m: "Error writing palette info".to_string() });
            };
        }
        for _ in 0..count {
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

    fn validate_size(&self, w: usize, h: usize) -> Result<(), ImageError> {
        if self.width != w || self.height != h || self.bits_per_pixel != 8 {
            return Err(ImageError::L2Size);
        }
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
                self.save_raw(name, PalettePlacement::Omit)
            }
            ImageType::Slr => {
                self.validate_size(128, 96)?;
                self.save_raw(name, PalettePlacement::Omit)
            }
        }
    }
}