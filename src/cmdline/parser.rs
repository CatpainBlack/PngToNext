use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

use argparse::{ArgumentParser, Store, StoreConst, StoreTrue};

use crate::cmdline::{Options, PalettePlacement};
use crate::image::ImageType;
use crate::primitives::rectangle::Rectangle;

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

    fn parse_crop(&mut self) -> &mut Options {
        if !self.crop.is_empty() {
            let nums: Vec<&str> = self.crop.split(',').collect();
            if nums.len() != 4 {
                println!("Crop must have 4 parameters");
                exit(1);
            }

            self.crop_to = Some(Rectangle {
                left: Options::parse_number(nums[0], "Crop left value is not a number"),
                top: Options::parse_number(nums[1], "Crop top value is not a number"),
                width: Options::parse_number(nums[2], "Crop width value is not a number"),
                height: Options::parse_number(nums[3], "Crop height value is not a number"),
            });
        }
        self
    }

    fn check_output(&mut self) -> &mut Options {
        if self.out_file_name.is_empty() {
            let mut path = PathBuf::from(&self.png_file_name);
            let ext = match self.output_type {
                ImageType::Raw => "bin",
                ImageType::Nxi => "nxi",
                ImageType::Asm => "asm",
                ImageType::Pal => "pal",
                ImageType::Npl => "npl",
                ImageType::Sl2 => "sl2",
                ImageType::Slr => "slr"
            };
            path.set_extension(ext);
            self.out_file_name = match path.file_name() {
                None => String::from("out"),
                Some(s) => s.to_string_lossy().to_string()
            };
        }
        self
    }

    pub fn parse() -> Options {
        let mut options = Options {
            png_file_name: "".to_string(),
            out_file_name: "".to_string(),
            output_type: ImageType::Raw,
            pal_priority: false,
            crop_to: None,
            pal_placement: PalettePlacement::Omit,
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
                .add_option(&["-n", "--nxi"], StoreConst(ImageType::Nxi), "Save the the image as a NXI file")
                .add_option(&["-a", "--asm"], StoreConst(ImageType::Asm), "Save data as assembly source")
                .add_option(&["-p", "--pal"], StoreConst(ImageType::Pal), "Save the palette as a .pal file")
                .add_option(&["-N", "--npl"], StoreConst(ImageType::Npl), "Save the palette as a .npl file")
                .add_option(&["-s", "--slr"], StoreConst(ImageType::Slr), "Save the the image as a slr file");

            parser.refer(&mut options.pal_placement) // only valid for raw output type
                .add_option(&["-P", "--prepend-palette"], StoreConst(PalettePlacement::Prepend), "Place palette data at the start of the file")
                .add_option(&["-A", "--append-palette"], StoreConst(PalettePlacement::Append), "Place palette data at the start of end file")
                .add_option(&["-O", "--no-palette"], StoreConst(PalettePlacement::Omit), "Do not include any palette data");

            parser.refer(&mut options.verbose)
                .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");

            parser.parse_args_or_exit();
        }
        options
            .parse_crop()
            .check_output();

        options
    }
}

