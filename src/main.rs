use crate::cmdline::Options;
use crate::convert::Palette;
use crate::png::Png;
use crate::png::PngError;

mod cmdline;
mod png;
mod convert;

fn main() -> Result<(), PngError> {
	let options = Options::parse();

	println!("File Name: {}", options.png_file_name);

	let mut png = Png::new();
	png.load(&options.png_file_name)?;

	if options.verbose {
		println!("Image: '{}', {}px x {}px [{} bits]", options.png_file_name, png.width, png.height, png.bit_depth);
		if !png.palette.is_empty() {
			println!("Palette entries: {}", png.palette.len());
		}
		println!("Output Type: {}", options.output_type);
		if let Some(c) = options.crop_to {
			println!("Crop: left={},top={} width={},height={}", c.left, c.top, c.width, c.height);
		}
	}

	match options.bits {
		8 => println!("{}", png.eight_bit_palette().iter().map(|p| format!("0x{:02X}", p)).collect::<Vec<String>>().join(",")),
		9 => println!("{}", png.nine_bit_palette().iter().map(|p| format!("0x{:04X}", p)).collect::<Vec<String>>().join(",")),
		_ => println!("Unknow bit depth")
	}

	Ok(())
}
