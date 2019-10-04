use crate::image::ImageError;

impl std::convert::From<std::io::Error> for ImageError {
    fn from(e: std::io::Error) -> Self {
        ImageError::IOError { m: e.to_string() }
    }
}
