use clap::Parser;

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

    let qr_data = match qr_decode::decode_to_bytes(&img.into()) {
        Ok(data) => data,
        Err(err) => print_error_and_exit(&format!("Failed to decode QR code: {err}")),
    };

    dbg!(qr_data);
}
