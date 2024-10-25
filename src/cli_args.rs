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
    #[clap(flatten)]
    pub file_format: ExportFileFormats,

    #[arg(
        short,
        long,
        help = "The file to output to. If unset or set to '-' the exported data will be printed to stdout",
        default_value = "-"
    )]
    pub output_file: std::path::PathBuf,

    #[arg(short, long, default_value = "base32")]
    pub secret_format: SecretFormat,

    #[arg(
        short,
        long,
        help = "Which token types to export",
        default_value = "totp"
    )]
    pub token_types: TokenTypes,

    #[arg(long, help = "Prettify JSON output")]
    pub pretty_json: bool,
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
pub struct ExportFileFormats {
    #[arg(short, long, help = "Export in JSON format")]
    pub json: bool,

    #[arg(short, long, help = "Export in text format")]
    pub text: bool,
}

#[derive(clap::ValueEnum, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SecretFormat {
    #[value(help = "Export the base32 secret only")]
    Base32,

    #[value(help = "Export the full OTP URL (where available)")]
    OtpUrl,
}

#[derive(clap::ValueEnum, Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenTypes {
    Totp,
    Hotp,
    All,
}
