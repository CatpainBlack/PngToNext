use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};
use rgb::FromSlice;

use crate::png::{Chunk, ColourType, Png};
use crate::png::PngError;

impl Png {
	fn read_header(file: &mut File) -> Result<(), PngError> {
		let png_header = [0x89u8, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
		let mut file_header = [0u8; 8];

		if file.seek(SeekFrom::Start(0)).is_err() {
			return Err(PngError::HeadError);
		}

		if file.read(&mut file_header).is_err() {
			return Err(PngError::HeadError);
		}

		if !png_header.eq(&mut file_header) {
			return Err(PngError::HeadError);
		}

		Ok(())
	}

	fn read_chunk(file: &mut File) -> Result<Option<Chunk>, PngError> {
		let mut buffer = [0u8, 0, 0, 0];

		match file.read(&mut buffer) {
			Ok(0) => return Ok(None),
			Ok(n) => Ok(n),
			Err(_) => Err(PngError::ReadError)
		}?;

		let length = match Cursor::new(buffer).read_u32::<BigEndian>() {
			Ok(len) => len as usize,
			Err(_) => return Err(PngError::ReadError),
		};

		if file.read(&mut buffer).is_err() {
			return Err(PngError::ReadError);
		};

		let chunk_type = match String::from_utf8(buffer.to_vec()) {
			Ok(ct) => ct,
			Err(_e) => return Err(PngError::ChunkError),
		};

		let mut data = vec![0u8; length];
		if file.read_exact(&mut data).is_err() {
			return Err(PngError::ReadError);
		}

		if file.read(&mut buffer).is_err() {
			return Err(PngError::ReadError);
		};

		let crc = match Cursor::new(buffer).read_u32::<BigEndian>() {
			Ok(read) => Ok(read as usize),
			Err(_) => Err(PngError::ReadError),
		}?;

		Ok(Some(Chunk {
			length,
			chunk_type,
			crc,
			data,
		}))
	}

	fn read_ihdr(&mut self, chunk: &Chunk) -> Result<(), PngError> {
		let mut cur = Cursor::new(&chunk.data);
		self.width = match cur.read_u32::<BigEndian>() {
			Ok(n) => Ok(n as usize),
			Err(_) => Err(PngError::ReadError)
		}?;

		self.height = match cur.read_u32::<BigEndian>() {
			Ok(n) => Ok(n as usize),
			Err(_) => Err(PngError::ReadError)
		}?;

		self.bit_depth = match cur.read_u8() {
			Ok(n) => Ok(n),
			Err(_) => Err(PngError::ReadError)
		}?;

		self.colour_type = match cur.read_u8() {
			Ok(0) => ColourType::GrayScale,
			Ok(2) => ColourType::Rgb,
			Ok(3) => ColourType::Palette,
			Ok(4) => ColourType::GrayScaleAlpha,
			Ok(6) => ColourType::RGBA,
			_ => return Err(PngError::UnsupportedColourDepth)
		};

		self.compression_method = match cur.read_u8() {
			Ok(n) => Ok(n),
			Err(_) => Err(PngError::ReadError)
		}?;

		self.filter_method = match cur.read_u8() {
			Ok(n) => Ok(n),
			Err(_) => Err(PngError::ReadError)
		}?;

		self.interlace_method = match cur.read_u8() {
			Ok(n) => Ok(n),
			Err(_) => Err(PngError::ReadError)
		}?;

		if self.interlace_method > 0 {
			return Err(PngError::InterlaceUnsupported);
		}

		Ok(())
	}

	pub fn read_idat(&mut self, chunk: &Chunk) -> Result<(), PngError> {
		if self.compression_method == 0 {
			let pixels = inflate::inflate_bytes_zlib(&chunk.data).unwrap();
			let w = self.width * self.bit_depth as usize / 8;
			for f in 0..self.height as usize {
				let start = f * (w as usize + 1usize) + 1;
				let row = &pixels[start..start + w as usize];
				self.image.extend_from_slice(&row)
			}
		} else {
			self.image = chunk.data.clone();
		}
		Ok(())
	}

	pub fn read_trns(&mut self, chunk: &Chunk) -> Result<(), PngError> {
		let index = chunk.data
			.iter()
			.enumerate()
			.find(|i| i.1 == &0u8)
			.unwrap_or((0usize, &0u8))
			.0 as u8;
		self.transparency_index = index;
		Ok(())
	}

	pub fn load(&mut self, file_path: &str) -> Result<(), PngError> {
		let mut file = match File::open(file_path) {
			Ok(file) => Ok(file),
			Err(_) => Err(PngError::Open)
		}?;

		Png::read_header(&mut file)?;

		while let Some(chunk) = Png::read_chunk(&mut file)? {
			match chunk.chunk_type.as_str() {
				"IHDR" => self.read_ihdr(&chunk)?,
				"PLTE" => self.palette = Vec::from(chunk.data.as_rgb()),
				"IDAT" => self.read_idat(&chunk)?,
				"tRNS" => self.read_trns(&chunk)?,
				"IEND" => break,
				_ => {
					println!("Unknown chunk type: {}", chunk.chunk_type);
				}
			}
			//println!("Chunk: {} {} {}", chunk.chunk_type, chunk.crc, chunk.length);
		}
		Ok(())
	}
}