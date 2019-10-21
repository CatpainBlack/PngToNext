use crate::convert::Process;
use crate::primitives::rectangle::Rectangle;
use crate::png::Png;
use crate::png::PngError;

impl Process for Png {
    fn copy_rect(self, mut rect: Rectangle) -> Result<Png, PngError> {
        let mut r = Rectangle {
            left: 0,
            top: 0,
            width: self.width as isize,
            height: self.height as isize,
        };
        if !rect.fits_in(&mut r) {
            return Err(PngError::InvalidCrop);
        }

        let mut copy = Png {
            width: rect.width as usize,
            height: rect.height as usize,
            colour_type: self.colour_type,
            compression_method: self.compression_method,
            filter_method: self.filter_method,
            interlace_method: self.interlace_method,
            palette: self.palette.clone(),
            image: vec![],
            transparency_index: self.transparency_index,
            bit_depth: self.bit_depth,
        };

        let mut row_width = self.width;
        row_width = match self.bit_depth {
            1 => row_width / 8,
            2 => row_width / 4,
            4 => row_width / 2,
            8 => row_width,
            16 => row_width * 2,
            24 => row_width * 3,
            32 => row_width * 4,
            _ => row_width
        };

        for row in rect.top as usize..rect.top as usize + rect.height as usize {
            let pos = (rect.left as usize + (row * row_width as usize)) as usize;
            let v = &self.image[pos..pos + rect.width as usize].to_vec();
            copy.image.append(v.clone().as_mut());
        }

        Ok(copy)
    }
}