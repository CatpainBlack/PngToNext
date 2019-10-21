use crate::image::{Block, BlockSet, Image};
use crate::primitives::rectangle::Rectangle;

impl BlockSet {
    pub fn new(tile_size: Rectangle) -> BlockSet {
        BlockSet {
            blocks: Default::default(),
            tile_size: tile_size.clone(),
        }
    }

    pub fn count(&self) -> usize {
        self.blocks.len()
    }

    fn add_block(&mut self, block: Block, transform: bool) {
        if self.blocks.contains_key(&block.hash) {
            return;
        } else if !transform {
            return;
        }
        for (_, b) in &self.blocks {
            if b.hashes.contains(&block.hash) {
                return;
            }
        }
        self.blocks.insert(block.hash.clone(), block);
    }

    pub fn process(&mut self, img: &Image, transform: bool) {
        let image_bounds = img.rect();
        for row in (0..image_bounds.height).step_by(self.tile_size.height as usize) {
            for col in (0..image_bounds.width).step_by(self.tile_size.width as usize) {
                if let Some(block) = Block::grab_from(&img, col, row, self.tile_size.width as u8) {
                    self.add_block(block, transform);
                }
            }
        }
    }
}