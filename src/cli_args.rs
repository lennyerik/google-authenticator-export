#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg()]
    pub image_path: std::path::PathBuf,

    #[arg(short, long)]
    pub quiet: bool,

    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum Subcommand {
    #[command(about = "Print metadata about the data encoded in the QR code")]
    Info,

    #[command(about = "Extract and print the OTP secrets")]
    Extract,

    #[command(about = "Export the OTP secrets to different file formats")]
    Export(ExportArgs),
}

#[derive(clap::Parser, Debug)]
pub struct ExportArgs {
    #[arg(short, long)]
    pub raw_json: bool,
}
