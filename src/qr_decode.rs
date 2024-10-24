#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no QR code found")]
    NoQRCodeFound,

    #[error("multiple QR codes found")]
    MultipleQRCodesFound,

    #[error("QR code decoding failed")]
    DecodingFailed,
}

pub fn decode_to_bytes(img: &image::GrayImage) -> Result<Vec<u8>, Error> {
    let mut decoder = quircs::Quirc::default();
    let mut codes = decoder
        .identify(img.width() as usize, img.height() as usize, img)
        .flatten();

    let code = codes.next().ok_or(Error::NoQRCodeFound)?;
    if codes.next().is_some() {
        return Err(Error::MultipleQRCodesFound);
    }

    let data = code.decode().map_err(|_| Error::DecodingFailed)?;
    Ok(data.payload)
}
