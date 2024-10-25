#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    #[error("no QR code found")]
    NoQRCodeFound,

    #[error("multiple QR codes found")]
    MultipleQRCodesFound,

    #[error("QR code decoding failed")]
    DecodingFailed,

    #[error("QR code data is not ASCII")]
    DataNotASCII,
}

pub fn decode_to_string(img: &image::GrayImage) -> Result<String, Error> {
    let mut decoder = quircs::Quirc::default();
    let mut codes = decoder
        .identify(img.width() as usize, img.height() as usize, img)
        .flatten();

    let code = codes.next().ok_or(Error::NoQRCodeFound)?;
    if codes.next().is_some() {
        return Err(Error::MultipleQRCodesFound);
    }

    let data = code.decode().map_err(|_| Error::DecodingFailed)?;
    String::from_utf8(data.payload).map_err(|_| Error::DataNotASCII)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    macro_rules! load_img {
        ($path:literal, $format:ident) => {{
            let img_bytes = include_bytes!($path);
            let mut reader = image::ImageReader::new(Cursor::new(img_bytes));
            reader.set_format(image::ImageFormat::$format);
            reader.decode().unwrap()
        }};
    }

    #[test]
    fn test_decode_to_string() {
        let img = load_img!("test_imgs/simple_text_qr_code.gif", Gif);
        assert_eq!(decode_to_string(&img.into()), Ok("test".into()));
    }

    #[test]
    fn test_decode_to_string_not_a_qr_code() {
        let img = load_img!("test_imgs/not_a_qr_code.jpg", Jpeg);
        assert_eq!(decode_to_string(&img.into()), Err(Error::NoQRCodeFound));
    }

    #[test]
    fn test_decode_to_string_two_qr_codes() {
        let img = load_img!("test_imgs/two_qr_codes.gif", Gif);
        assert_eq!(
            decode_to_string(&img.into()),
            Err(Error::MultipleQRCodesFound)
        );
    }

    #[test]
    fn test_decode_to_string_non_utf8_qr_code() {
        let img = load_img!("test_imgs/non_utf8_qr_code.jpg", Jpeg);
        assert_eq!(decode_to_string(&img.into()), Err(Error::DataNotASCII));
    }
}
