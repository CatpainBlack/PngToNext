#[derive(Debug)]
pub struct Rectangle {
    pub left: isize,
    pub top: isize,
    pub width: isize,
    pub height: isize,
}

impl Rectangle {
    pub fn square(x: isize, y: isize, size: isize) -> Rectangle {
        Rectangle {
            left: x,
            top: y,
            width: size,
            height: size,
        }
    }

    pub fn fits_in(&mut self, parent: &mut Rectangle) -> bool {
        if self.left + self.width > parent.width {
            return false;
        }
        if self.top + self.height > parent.height {
            return false;
        }
        true
    }
}