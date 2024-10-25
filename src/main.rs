use clap::Parser;

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

    let payload = qr_decode::parse_qr_payload(&qr_data).unwrap_or_else(|e| {
        print_error_and_exit(
            q,
            &format!(
                concat!(
                    "Parsing error: {}\n",
                    "Are you sure the QR code is a valid Google Authenticator App export code?"
                ),
                e
            ),
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
