use crate::cmdline::Options;
use argparse::{ArgumentParser, Store, StoreTrue};

impl Options {
	pub fn parse() -> Options {
		let mut options = Options::default();
		{
			let mut parser = ArgumentParser::new();
			parser.set_description("Png to ZX Next image converter");

			parser.refer(&mut options.png_file_name)
				.add_argument("png", Store, "Source image file")
				.required();

			parser.refer(&mut options.out_file_name)
				.add_argument("file", Store, "Output file");
				//.required();

			parser.refer(&mut options.verbose)
				.add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");

			parser.parse_args_or_exit();
		}
		options
	}
}