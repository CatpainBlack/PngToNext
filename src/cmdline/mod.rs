use std::fmt::{Display, Error, Formatter};

pub mod parser;

#[derive(Clone)]
pub enum OutputType {
	Raw,
	Nxi,
	Asm,
	Pal,
	Npl,
	Sl2,
}

pub struct Rectangle {
	pub left: isize,
	pub top: isize,
	pub width: isize,
	pub height: isize,
}

pub struct Options {
	// Files
	pub png_file_name: String,
	pub out_file_name: String,

	//
	pub output_type: OutputType,
	pub pal_priority: bool,
	pub bits: u8,
	pub crop_to: Option<Rectangle>,

	// Internal
	crop: String,

	// Behaviour
	pub verbose: bool,
}

impl Display for OutputType {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		match self {
			OutputType::Raw => write!(f, "Raw Data"),
			OutputType::Nxi => write!(f, "NXI"),
			OutputType::Asm => write!(f, "Assembler Source"),
			OutputType::Pal => write!(f, "Pal"),
			OutputType::Npl => write!(f, "Npl"),
			OutputType::Sl2 => write!(f, "Sl2"),
		}
	}
}
