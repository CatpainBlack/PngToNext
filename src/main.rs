use crate::cmdline::Options;
use crate::convert::Process;
use crate::png::Png;
use crate::image::{Image, ImageType};
use crate::image::PixelFormat;
use crate::errors::CmdError;

mod cmdline;
mod png;
mod convert;
mod image;
mod errors;

fn main() -> Result<(), CmdError> {
    let options = Options::parse();

    println!("File Name: {}", options.png_file_name);

    let mut png = Png::new();
    png.load(&options.png_file_name)?;

    if options.verbose {
        println!("Loaded image: '{}', {}px x {}px [{} bits]", options.png_file_name, png.width, png.height, png.bit_depth);
        if !png.palette.is_empty() {
            println!("Palette entries: {}", png.palette.len());
        }
        println!("Output Type: {}", options.output_type);
    }

    if let Some(c) = options.crop_to {
        if options.verbose {
            println!("Crop: left={},top={} width={},height={}", c.left, c.top, c.width, c.height);
        }
        png = png.copy_rect(c)?;
    }

    let mut img8 = Image::from(&png);
    let mut img4 = Image::from(&png);

    img8.resample(PixelFormat::EightBit)?;
    img4.resample(PixelFormat::FourBit)?;

    img8.save(ImageType::Nxi, "test.nxi")?;
    img8.save(ImageType::Sl2, "test.sl2")?;
    img8.save(ImageType::Raw, "test.raw")?;
    img8.save(ImageType::Pal, "test.pal")?;
    img8.save(ImageType::Npl, "test.npl")?;

    Ok(())
}
