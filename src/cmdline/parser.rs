use std::process::exit;
use std::str::FromStr;

use argparse::{ArgumentParser, Store, StoreConst, StoreTrue};

use crate::cmdline::Options;
use crate::image::ImageType;
use crate::image::rectangle::Rectangle;

impl Options {
	fn parse_number(n: &str, error: &str) -> isize {
		match isize::from_str(n) {
			Ok(num) => num,
			Err(_) => {
				println!("{}", error);
				exit(1);
			}
		}
	}

	pub fn parse() -> Options {
		let mut options = Options {
			png_file_name: "".to_string(),
			out_file_name: "".to_string(),
			output_type: ImageType::Raw,
			pal_priority: false,
			bits: 9,
			crop_to: None,
			crop: "".to_string(),
			verbose: false,
		};
		{
			let mut parser = ArgumentParser::new();
			parser.set_description("Png to ZX Next image converter");

			parser.refer(&mut options.png_file_name)
				.add_argument("png", Store, "Source image file")
				.required();

			parser.refer(&mut options.out_file_name)
				.add_argument("output", Store, "Converted file");

			parser.refer(&mut options.crop)
				.add_option(&["-c", "--crop"], Store, "Crop image (left,top,width,height)");

			parser.refer(&mut options.output_type)
				.add_option(&["-r", "--raw"], StoreConst(ImageType::Raw), "Save the image as raw data")
				.add_option(&["-2", "--sl2"], StoreConst(ImageType::Sl2), "Save the image as a SL2 data (raw layer 2 image data)")
				.add_option(&["-n", "--nxi"], StoreConst(ImageType::Nxi), "the the image as a NXI file")
				.add_option(&["-a", "--asm"], StoreConst(ImageType::Asm), "Save data as assembly source")
				.add_option(&["-p", "--pal"], StoreConst(ImageType::Pal), "Save the palette as a .pal file")
				.add_option(&["-N", "--npl"], StoreConst(ImageType::Npl), "Save the palette as a .npl file");

			parser.refer(&mut options.bits)
				.add_option(&["-8", "--8bit"], StoreConst(8), "Save palette data in 8 bit format")
				.add_option(&["-9", "--9bit"], StoreConst(9), "Save palette data in 9 bit format");

			parser.refer(&mut options.pal_priority)
				.add_option(&["-P", "--pixel-priority"], StoreTrue, "Set the pixel priority of each palette entry");

			parser.refer(&mut options.verbose)
				.add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");

			parser.parse_args_or_exit();
		}

		if !options.crop.is_empty() {
			let nums: Vec<&str> = options.crop.split(',').collect();
			if nums.len() != 4 {
				println!("Crop must have 4 parameters");
				exit(1);
			}

			options.crop_to = Some(Rectangle {
				left: Options::parse_number(nums[0], "Crop left value is not a number"),
				top: Options::parse_number(nums[1], "Crop top value is not a number"),
				width: Options::parse_number(nums[2], "Crop width value is not a number"),
				height: Options::parse_number(nums[3], "Crop height value is not a number"),
			});
		}

		options
	}
}

