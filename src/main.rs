use clap::Parser;
use prost::Message;

const QR_PREFIX: &str = "otpauth-migration://offline?data=";
const INVALID_DATA_MSG: &str =
    "Are you sure the QR code is a valid Google Authenticator App export code?";

mod authenticator_export {
    #![allow(clippy::all)]
    #![allow(clippy::pedantic)]
    #![allow(clippy::nursery)]
    include!(concat!(env!("OUT_DIR"), "/authenticator.export.rs"));
}
mod cli_args;
mod commands;
mod qr_decode;

fn print_error_and_exit(quiet: bool, err: &str) -> ! {
    if !quiet {
        eprintln!("{err}");
    }
    std::process::exit(1)
}

fn main() {
    let args = cli_args::Args::parse();
    let q = args.quiet;

    let img_file = image::ImageReader::open(&args.image_path).unwrap_or_else(|e| {
        print_error_and_exit(
            q,
            &format!(
                "Failed to open file '{}': {e}",
                args.image_path.to_string_lossy(),
            ),
        )
    });

    let img = img_file.decode().unwrap_or_else(|e| {
        print_error_and_exit(
            q,
            &format!(
                "Failed to read image file '{}': {e}",
                args.image_path.to_string_lossy()
            ),
        )
    });

    let qr_data = qr_decode::decode_to_string(&img.into())
        .unwrap_or_else(|e| print_error_and_exit(q, &format!("Failed to decode QR code: {e}")));

    let url_data = qr_data
        .strip_prefix("otpauth-migration://offline?data=")
        .unwrap_or_else(|| {
            print_error_and_exit(
                q,
                &format!("Failed to find {QR_PREFIX} URL in QR code.\n{INVALID_DATA_MSG}"),
            )
        });

    let url_decoded_data = urlencoding::decode(url_data).unwrap_or_else(|_| {
        print_error_and_exit(
            q,
            &format!(
            "Migration payload URL data contained encoded non-UTF8 characters\n{INVALID_DATA_MSG}"
        ),
        )
    });

    let binary_data = data_encoding::BASE64
        .decode(url_decoded_data.as_bytes())
        .unwrap_or_else(|e| {
            print_error_and_exit(
                q,
                &format!(
                    "Failed to parse migration payload data as base64: {e}\n{INVALID_DATA_MSG}"
                ),
            )
        });

    let payload = authenticator_export::MigrationPayload::decode(binary_data.as_slice())
        .unwrap_or_else(|e| {
            print_error_and_exit(
                q,
                &format!("Failed to decode binary QR code data: {e}\n{INVALID_DATA_MSG}"),
            )
        });

    match args.subcommand {
        cli_args::Subcommand::Info => {
            commands::print_info(&payload).expect("Failed to write to stdout");
        }
        cli_args::Subcommand::Extract => {
            commands::extract_tokens(&payload).expect("Failed to write to stdout");
        }
        cli_args::Subcommand::Export(export_args) => {
            commands::export_tokens(&payload, &export_args).unwrap_or_else(|e| {
                print_error_and_exit(q, &format!("Failed to export data: {e}"))
            });
        }
    }
}
