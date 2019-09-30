use crate::image::rectangle::Rectangle;
use crate::png::Png;
use crate::png::PngError;

mod palette_entry;
mod palette;
mod process_impl;

pub trait PaletteEntry {
	fn as_8bit(&self) -> u8;
	fn as_9bit(&self) -> u16;
}

pub trait Palette {
	fn eight_bit_palette(&self) -> Vec<u8>;
	fn nine_bit_palette(&self) -> Vec<u16>;
}

pub trait Process {
	fn copy_rect(self, rect: Rectangle) -> Result<Png, PngError>;
}