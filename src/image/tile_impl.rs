extern crate sha1;

use sha1::{Sha1, Digest};
use crate::image::{Block, Image, Hash};
use crate::primitives::rectangle::Rectangle;

impl Hash for Vec<Vec<u8>> {
    fn hash(&self) -> String {
        let mut hasher = Sha1::new();
        for row in self {
            hasher.input(row.as_slice());
        }
        let hash: Vec<String> = hasher.result().iter().map(|x| format!("{:02x}", x)).collect();
        hash.join("")
    }
}

impl Block {
    pub fn transform_hash(&self, flip: bool, mirror: bool, rotate: bool) -> String {
        let mut pixels = self.pixels.clone();
        if rotate {
            for i in 0..self.size {
                for j in 1..self.size {
                    pixels[i][j] = self.pixels[self.size - j][i];
                }
            }
        }
        if flip {
            pixels.reverse();
        }
        if mirror {
            for row in &mut pixels {
                row.reverse();
            }
        }
        pixels.hash()
    }

    fn calculate_hashes(&mut self) {
        self.hash = self.pixels.hash();
        self.hashes.push(self.transform_hash(false, false, true));
        self.hashes.push(self.transform_hash(false, true, false));
        self.hashes.push(self.transform_hash(false, true, true));
        self.hashes.push(self.transform_hash(true, false, false));
        self.hashes.push(self.transform_hash(true, false, true));
        self.hashes.push(self.transform_hash(true, true, false));
        self.hashes.push(self.transform_hash(true, true, true));
    }

    pub fn grab_from(i: &Image, x: isize, y: isize, s: u8) -> Option<Block> {
        let mut region = Rectangle::square(x, y, s as isize);
        let fits = region.fits_in(&mut i.rect());
        if i.bits_per_pixel != 8 || !fits {
            return None;
        }

        let mut pixels: Vec<Vec<u8>> = vec![];
        let top_row = region.top;
        let bottom_row = region.top + region.height;
        for row in top_row..bottom_row {
            let start = (row + region.left) as usize;
            let end = (start + region.width as usize) as usize;
            let copied = i.pixels[start..end].to_vec().clone();
            pixels.push(copied);
        }
        let mut b = Block {
            size: s as usize,
            pixels,
            hash: String::new(),
            hashes: vec![],
        };
        b.calculate_hashes();
        Some(b)
    }
}