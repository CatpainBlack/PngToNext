use crate::png::Png;
use crate::image::{Image, PixelFormat, ImageType};
use crate::convert::PaletteEntry;
use crate::image::ImageError;
use std::io::{Cursor, BufWriter, Write};
use self::bitstream_io::{BitReader, LittleEndian, BitWriter};
use std::fs::File;
use byteorder::WriteBytesExt;

extern crate bitstream_io;

impl From<&Png> for Image {
    fn from(png: &Png) -> Self {
        Image {
            height: png.height,
            width: png.width,
            bits_per_pixel: png.bit_depth as u32,
            pixels: png.image.clone(),
            transparency: png.transparency_index,
            rgb_pal: png.palette.clone(),
        }
    }
}

impl Image {
//    fn convert_palette(&mut self, pal_type: PaletteType) {
//        match pal_type {
//            PaletteType::RGB332 => {
////                for entry in &self.rgb_pal {
////                    self.palette.push(entry.as_9bit());
////                }
//            }
//            PaletteType::RGB333 => {
////                for entry in &self.rgb_pal {
////                    self.palette.push(entry.as_8bit() as u16);
////                }
//            }
//        }
//    }

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

    fn save_raw(&mut self, name: &str, append_palette: bool) -> Result<(), ImageError> {
        let file = File::create(name)?;
        let mut writer = BufWriter::new(file);
        if writer.write_all(&self.pixels).is_err() {
            return Err(ImageError::IOError { m: "Error writing pixel data".to_string() });
        };
        if append_palette {
            for rgb in &self.rgb_pal {
                if writer.write_u16::<byteorder::LittleEndian>(rgb.as_9bit()).is_err() {
                    return Err(ImageError::IOError { m: "Error writing palette info".to_string() });
                };
            }
        }
        Ok(())
    }

    fn save_pal(&mut self, name: &str, transparency: Option<u8>) -> Result<(), ImageError> {
        let file = File::create(name)?;
        let mut writer = BufWriter::new(file);
        let count = 256 - self.rgb_pal.len();
        for rgb in &self.rgb_pal {
            if writer.write_u16::<byteorder::LittleEndian>(rgb.as_9bit()).is_err() {
                return Err(ImageError::IOError { m: "Error writing palette info".to_string() });
            };
        }
        for _ in 0..count {
            if writer.write_u16::<byteorder::LittleEndian>(0).is_err() {
                return Err(ImageError::IOError { m: "Error writing palette info".to_string() });
            };
        }
        if let Some(b) = transparency {
            if writer.write_u8(b).is_err() {
                return Err(ImageError::IOError { m: "Error writing transparency info".to_string() });
            };
        }
        Ok(())
    }

    pub fn save(&mut self, image_type: ImageType, name: &str) -> Result<(), ImageError> {
        match image_type {
            ImageType::Raw => self.save_raw(name, false),
            ImageType::Nxi => self.save_raw(name, true),
            ImageType::Asm => unimplemented!(),
            ImageType::Pal => self.save_pal(name, None),
            ImageType::Npl => self.save_pal(name, Some(self.transparency)),
            ImageType::Sl2 => self.save_raw(name, false)
        }
    }
}