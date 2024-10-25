use prost::Message;

use crate::authenticator_export;

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DecodingError {
    #[error("no QR code found")]
    NoQRCodeFound,

    #[error("multiple QR codes found")]
    MultipleQRCodesFound,

    #[error("QR code decoding failed")]
    DecodingFailed,

    #[error("QR code data is not UTF8")]
    InvalidUtf8,
}

pub fn decode_to_string(img: &image::GrayImage) -> Result<String, DecodingError> {
    let mut decoder = quircs::Quirc::default();
    let mut codes = decoder
        .identify(img.width() as usize, img.height() as usize, img)
        .flatten();

    let code = codes.next().ok_or(DecodingError::NoQRCodeFound)?;
    if codes.next().is_some() {
        return Err(DecodingError::MultipleQRCodesFound);
    }

    let data = code.decode().map_err(|_| DecodingError::DecodingFailed)?;
    String::from_utf8(data.payload).map_err(|_| DecodingError::InvalidUtf8)
}

const QR_PREFIX: &str = "otpauth-migration://offline?data=";

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub enum ParsingError {
    #[error("failed to find '{QR_PREFIX}' URL in QR code data")]
    InvalidPrefix,

    #[error("migration payload URL data contained encoded non-UTF8 characters")]
    InvalidUtf8,

    #[error("failed to parse migration payload data as base64: `{0}`")]
    InvalidBase64(#[from] data_encoding::DecodeError),

    #[error("failed to decode binary QR code data into protobuf schema: `{0}`")]
    ProtobufDecodingFailed(#[from] prost::DecodeError),
}

pub fn parse_qr_payload(
    data: &str,
) -> Result<authenticator_export::MigrationPayload, ParsingError> {
    let url_data = data
        .strip_prefix(QR_PREFIX)
        .ok_or(ParsingError::InvalidPrefix)?;

    let url_decoded_data = urlencoding::decode(url_data).map_err(|_| ParsingError::InvalidUtf8)?;

    let binary_data = data_encoding::BASE64.decode(url_decoded_data.as_bytes())?;

    let payload = authenticator_export::MigrationPayload::decode(binary_data.as_slice())?;

    Ok(payload)
}

#[cfg(test)]
mod tests {
    use authenticator_export::MigrationPayload;

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
        let img = load_img!("test/img/simple_text_qr_code.gif", Gif);
        assert_eq!(decode_to_string(&img.into()), Ok("test".into()));
    }

    #[test]
    fn test_decode_to_string_not_a_qr_code() {
        let img = load_img!("test/img/not_a_qr_code.jpg", Jpeg);
        assert_eq!(
            decode_to_string(&img.into()),
            Err(DecodingError::NoQRCodeFound)
        );
    }

    #[test]
    fn test_decode_to_string_two_qr_codes() {
        let img = load_img!("test/img/two_qr_codes.gif", Gif);
        assert_eq!(
            decode_to_string(&img.into()),
            Err(DecodingError::MultipleQRCodesFound)
        );
    }

    #[test]
    fn test_decode_to_string_non_utf8_qr_code() {
        let img = load_img!("test/img/non_utf8_qr_code.jpg", Jpeg);
        assert_eq!(
            decode_to_string(&img.into()),
            Err(DecodingError::InvalidUtf8)
        );
    }

    #[test]
    fn test_parse_qr_payload() {
        const TEST_QR_PAYLOAD: &str = "otpauth-migration://offline?data=CikKCkhlbGxvId6tvu8SCHRlc3R1c2VyGgtleGFtcGxlLmNvbSABKAEwAgo8CgpXb3JsZCEgICAgEhx0ZXN0LWdpdGh1Yi51c2VyQGV4YW1wbGUuY29tGgpnaXRodWIuY29tIAEoATACCjcKCldvcmxkISAgICESD2Fub3RoZXJ0ZXN0dXNlchoSc29tZWV4YW1wbGVzZXJ2aWNlIAEoATACCjIKCldvcmxkISAgICISCmZvdXJ0aHVzZXIaEnNvbWVleGFtcGxlc2VydmljZSABKAEwAhACGAEgAA%3D%3D";
        let parsed_payload =
            MigrationPayload::decode(&include_bytes!("test/proto/test_data.bin")[..]).unwrap();
        assert_eq!(parse_qr_payload(TEST_QR_PAYLOAD), Ok(parsed_payload));
    }

    #[test]
    fn test_parse_qr_payload_invalid_prefix() {
        assert_eq!(
            parse_qr_payload("invalid prefix"),
            Err(ParsingError::InvalidPrefix)
        );
    }

    #[test]
    fn test_parse_qr_payload_invalid_utf8() {
        assert_eq!(
            parse_qr_payload(
                "otpauth-migration://offline?data=somepayload%00with%DE%13illegalchars"
            ),
            Err(ParsingError::InvalidUtf8)
        );
    }

    #[test]
    fn test_parse_qr_payload_invalid_base64() {
        let result =
            parse_qr_payload("otpauth-migration://offline?data=DEFINITELYN0#*($@)@_)T!BASE64");

        // `assert_matches!()` is still unstable
        assert!(matches!(result, Err(ParsingError::InvalidBase64(_))));
    }
}
