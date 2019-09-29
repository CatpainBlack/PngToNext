extern crate custom_error;

use custom_error::custom_error;
use rgb::RGB8;

mod png_loader;

pub enum ColourType {
	Unknown,
	GrayScale,
	Rgb,
	Palette,
	GrayScaleAlpha,
	RGBA,
}

pub struct Chunk {
	pub length: usize,
	pub chunk_type: String,
	pub data: Vec<u8>,
	pub crc: usize,
}

pub trait PngReader {
	fn read_header(&mut self) -> Result<bool, PngError>;
	fn read_chunk(&mut self) -> Result<Chunk, PngError>;
}

pub struct Png {
	pub width: usize,
	pub height: usize,
	pub colour_type: ColourType,
	pub compression_method: u8,
	pub filter_method: u8,
	pub interlace_method: u8,
	pub palette: Vec<RGB8>,
	pub image: Vec<u8>,
	pub transparency_index: u8,
	pub bit_depth: u8,
}

impl Png {
	pub fn new() -> Png {
		Png {
			width: 0,
			height: 0,
			colour_type: ColourType::Unknown,
			compression_method: 0,
			filter_method: 0,
			interlace_method: 0,
			palette: vec![],
			image: vec![],
			transparency_index: 0,
			bit_depth: 0,
		}
	}
}

custom_error! {pub PngError
	Open="Could not open PNG file",
	HeadError="Invalid PNG header",
	ReadError="PNG Read error",
	ChunkError="PNG Chunk error",
	InterlaceUnsupported="Interlaced PNGs are not supported",
	UnsupportedColourDepth="Unsupported colour depth",
}