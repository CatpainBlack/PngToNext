extern crate custom_error;

use custom_error::custom_error;
use rgb::RGB8;
use std::collections::HashMap;
use crate::primitives::rectangle::Rectangle;


mod img_from;
mod image_impl;
mod resample;
mod tile_impl;
mod from_impl;
mod display_impl;
mod hash_impl;
mod block_set_impl;

#[derive(Clone)]
pub enum ImageType {
    Raw,
    Nxi,
    Asm,
    Pal,
    Npl,
    Sl2,
    Slr,
}

pub enum PixelFormat {
    FourBit,
    EightBit,
}

pub struct Image {
    pub height: usize,
    pub width: usize,
    pub bits_per_pixel: u32,
    pub pixels: Vec<u8>,
    pub transparency: u8,
    pub next_palette: Vec<u16>,
    rgb_pal: Vec<RGB8>,
}

#[derive(Debug)]
pub struct Block {
    size: usize,
    pub pixels: Vec<Vec<u8>>,
    pub hash: String,
    pub hashes: Vec<String>,
}

pub struct BlockSet {
    pub blocks: HashMap<String, Block>,
    pub tile_size: Rectangle,
}

pub trait Hash {
    fn hash(&self) -> String;
}

custom_error! {pub ImageError
    BitDepth{bpp:u8}="Unsupported bit depth {bpp}",
    Resample="Could not resample pixel data",
    IOError{m:String}="IO Error {m}",
    L2Size="Image must be 256x192 8 bit indexed colour",
    PaletteRemap="Could not remap colours"
}

