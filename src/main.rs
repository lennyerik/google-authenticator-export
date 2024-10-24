use base64::Engine;
use clap::Parser;
use prost::Message;
use std::fmt::Write;

const QR_PREFIX: &str = "otpauth-migration://offline?data=";
const INVALID_DATA_MSG: &str =
    "Are you sure the QR code is a valid Google Authenticator App export code?";

mod authenticator_export {
    #![allow(clippy::all)]
    #![allow(clippy::pedantic)]
    #![allow(clippy::nursery)]
    include!(concat!(env!("OUT_DIR"), "/authenticator.export.rs"));
}
mod qr_decode;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CLIArgs {
    #[arg()]
    image_path: std::path::PathBuf,
}

fn print_error_and_exit(err: &str) -> ! {
    eprintln!("{err}");
    std::process::exit(1)
}

fn main() {
    let args = CLIArgs::parse();

    let img_file = image::ImageReader::open(&args.image_path).unwrap_or_else(|e| {
        print_error_and_exit(&format!(
            "Failed to open file '{}': {e}",
            args.image_path.to_string_lossy(),
        ))
    });

    let img = img_file.decode().unwrap_or_else(|e| {
        print_error_and_exit(&format!(
            "Failed to read image file '{}': {e}",
            args.image_path.to_string_lossy()
        ))
    });

    let qr_data = qr_decode::decode_to_string(&img.into())
        .unwrap_or_else(|e| print_error_and_exit(&format!("Failed to decode QR code: {e}")));

    let url_data = qr_data
        .strip_prefix("otpauth-migration://offline?data=")
        .unwrap_or_else(|| {
            print_error_and_exit(&format!(
                "Failed to find {QR_PREFIX} URL in QR code.\n{INVALID_DATA_MSG}"
            ))
        });

    let url_decoded_data = urlencoding::decode(url_data).unwrap_or_else(|_| {
        print_error_and_exit(&format!(
            "Migration payload URL data contained encoded non-UTF8 characters\n{INVALID_DATA_MSG}"
        ))
    });

    let binary_data = base64::prelude::BASE64_STANDARD
        .decode(url_decoded_data.as_ref())
        .unwrap_or_else(|e| {
            print_error_and_exit(&format!(
                "Failed to parse migration payload data as base64: {e}\n{INVALID_DATA_MSG}"
            ))
        });

    let payload = authenticator_export::MigrationPayload::decode(binary_data.as_slice())
        .unwrap_or_else(|e| {
            print_error_and_exit(&format!(
                "Failed to decode binary QR code data: {e}\n{INVALID_DATA_MSG}"
            ))
        });

    let contained_account_names =
        payload
            .otp_parameters
            .iter()
            .fold(String::new(), |mut curr, param| {
                writeln!(curr, "        {}", param.name).unwrap();
                curr
            });

    println!(
        concat!(
            "Detected migration payload:\n",
            "   version: {}\n",
            "   batch_size: {}\n",
            "   batch_index: {}\n",
            "   batch_id: {}\n",
            "   contained accounts:\n{}"
        ),
        payload.version,
        payload.batch_size,
        payload.batch_index,
        payload.batch_id,
        contained_account_names
    );
}
