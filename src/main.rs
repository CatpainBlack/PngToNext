use crate::cmdline::Options;
use crate::convert::Process;
use crate::errors::CmdError;
use crate::image::{Image, BlockSet};
use crate::image::PixelFormat;
use crate::png::Png;
use std::process::exit;
use crate::primitives::rectangle::Rectangle;

mod cmdline;
mod png;
mod convert;
mod image;
mod errors;
mod primitives;

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

    let mut img8 = Image::from(&png);
    img8.resample(PixelFormat::EightBit)?;

    let mut img4 = Image::from(&png);
    img4.resample(PixelFormat::FourBit)?;


    if options.verbose {
        println!("Processing map data...");
    }
    let mut blocks = BlockSet::new(Rectangle::square(0, 0, 16));
    blocks.process(&img8);
    if options.verbose {
        println!("Grabbed {} unique blocks", blocks.count());
    }

    if false {
        if options.verbose {
            println!("Saving image file: {}", options.out_file_name);
        }

        img8.save(options.output_type, &options.out_file_name)?;
    }
    Ok(())
}
