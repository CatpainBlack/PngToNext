use crate::convert::Palette;
use crate::convert::PaletteEntry;
use crate::png::Png;

impl Palette for Png {
	fn eight_bit_palette(&self) -> Vec<u8> {
		let pal: Vec<u8> = self.palette.iter().map(|p| p.as_8bit()).collect();
		pal
	}

	fn nine_bit_palette(&self) -> Vec<u16> {
		let pal: Vec<u16> = self.palette.iter().map(|p| p.as_9bit()).collect();
		pal
	}
}
