use custom_error::custom_error;
use crate::image::ImageError;
use crate::png::PngError;

custom_error! {pub CmdError
    ImageError  {source: ImageError}= "Image error {source}",
    PngError {source: PngError}  ="PNG Error {source}"
}

//impl From<ImageError> for CmdError {
//    fn from(e: ImageError) -> Self {
//        CmdError::ImageError { msg: e }
//    }
//}
//
//impl From<PngError> for CmdError {
//    fn from(e: PngError) -> Self {
//        CmdError::PngError { msg: e.to_string() }
//    }
//}