use crate::cmdline::Options;
use crate::png::Png;
use crate::png::PngError;

mod cmdline;
mod png;

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
	}

	Ok(())
}
