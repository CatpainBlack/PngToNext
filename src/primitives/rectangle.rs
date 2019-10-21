#[derive(Debug,Copy, Clone)]
pub struct Rectangle {
    pub left: isize,
    pub top: isize,
    pub width: isize,
    pub height: isize,
}


impl Rectangle {
    pub fn tile() -> Rectangle {
        Rectangle {
            left: 0,
            top: 0,
            width: 8,
            height: 8,
        }
    }

    pub fn sprite() -> Rectangle {
        Rectangle {
            left: 0,
            top: 0,
            width: 16,
            height: 16,
        }
    }

    pub fn square(left: isize, top: isize, size: isize) -> Rectangle {
        Rectangle {
            left,
            top,
            width: size,
            height: size,
        }
    }

    pub fn fits_in(&mut self, parent: &mut Rectangle) -> bool {
        if (self.top + self.height) > parent.height {
            return false;
        }
        if (self.left + self.width) > parent.width {
            return false;
        }
        true
    }
}