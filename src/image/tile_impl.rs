use crate::image::{Block, Image, Hash};
use crate::primitives::rectangle::Rectangle;

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

    pub fn grab_from(i: &Image, left: isize, top: isize, s: u8) -> Option<Block> {
        let mut region = Rectangle::square(left, top, s as isize);
        let fits = region.fits_in(&mut i.rect());
        if i.bits_per_pixel != 8 || !fits {
            return None;
        }

        let mut pixels: Vec<Vec<u8>> = vec![];

        let mut start: usize = region.left as usize + (region.top as usize * i.width);
        for _index in 0..region.height {
            pixels.push(i.pixels[start..start + region.width as usize].to_vec().clone());
            start += i.width;
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