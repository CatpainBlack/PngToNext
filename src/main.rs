use std::process::exit;

use crate::cmdline::Options;
use crate::convert::Process;
use crate::errors::CmdError;
use crate::image::{BlockSet, Image, ImageType};
use crate::image::PixelFormat;
use crate::image_converter::ImageConverter;
use crate::png::Png;
use crate::primitives::rectangle::Rectangle;

mod cmdline;
mod png;
mod convert;
mod image;
mod errors;
mod primitives;
mod image_converter;

fn main() {
    match convert_image() {
        Ok(_) => {
            exit(0);
        }
        Err(e) => {
            match e {
                CmdError::ImageError { source } => println!("{}", source),
                CmdError::PngError { source } => println!("{}", source),
            }
            exit(1);
        }
    }
}

fn convert_image() -> Result<(), CmdError> {
    let options = Options::parse();

    let mut png = Png::new();
    png.load(&options.png_file_name)?;

    if options.verbose {
        println!("Loaded image: '{}', {}px x {}px [{} bits]", options.png_file_name, png.width, png.height, png.bits_per_pixel().unwrap_or(0));
        if !png.palette.is_empty() {
            println!("Palette entries: {}", png.palette.len());
        }
        println!("Output Type: {}", options.output_type);
    }

    if let Some(c) = options.crop_to {
        if options.verbose {
            println!("Crop dimensions:\n\tleft:\t{}\n\ttop:\t{}\n\twidth:\t{}\n\theight:\t{}", c.left, c.top, c.width, c.height);
        }
        png = png.copy_rect(c)?;
    }

    match options.output_type {
        ImageType::Raw |
        ImageType::Nxi |
        ImageType::Pal |
        ImageType::Npl |
        ImageType::Sl2 |
        ImageType::Slr => {
            if options.verbose {
                println!("Saving image file: {}", options.out_file_name);
            }
            let converter = ImageConverter::new(&png)
                .output_format(options.output_type)
                .save_as(&options.out_file_name)?;
        }

        ImageType::Asm => unimplemented!(),
        ImageType::Spr => {
            println!("Sprites!");
        }
    }

    Ok(())
}
