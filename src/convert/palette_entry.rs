use rgb::RGB8;

use crate::convert::PaletteEntry;

impl PaletteEntry for RGB8 {
	fn as_8bit(&self) -> u8 {
		let r = (self.r as u16 * 7 / 255) & 7;
		let g = (self.g as u16 * 7 / 255) & 7;
		let b = (self.b as u16 * 3 / 255) & 7;
		(r << 5 | g & 7 << 2 | b & 7 >> 1) as u8
	}

	fn as_9bit(&self) -> u16 {
		let r = (self.r as u32 * 7 / 255) as u16 & 7;
		let b = (self.b as u32 * 7 / 255) as u16 & 7;
		let g = (self.g as u32 * 7 / 255) as u16 & 7;
		let b1 = (r << 5 | g << 2 | b >> 1) as u16;
		let b2 = (b & 1) as u16;
		b1 << 8 | b2
	}
}