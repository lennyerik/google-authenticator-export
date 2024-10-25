use crate::{
    authenticator_export::{
        migration_payload::{Algorithm, DigitCount, OtpParameters, OtpType},
        MigrationPayload,
    },
    cli_args,
};
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

impl OtpParameters {
    fn digits_numeric(&self) -> Option<u8> {
        match self.digits() {
            DigitCount::Unspecified => None,
            DigitCount::Six => Some(6),
            DigitCount::Eight => Some(8),
        }
    }

    fn otp_url(&self) -> Option<String> {
        if self.r#type() == OtpType::Totp {
            let mut url = "otpauth://totp/".to_string();
            write!(url, "{}?", urlencoding::encode(&self.name)).ok()?;
            write!(url, "issuer={}", urlencoding::encode(&self.issuer)).ok()?;

            write!(
                url,
                "&secret={}",
                data_encoding::BASE32.encode(&self.secret)
            )
            .ok()?;

            if let Some(digits) = self.digits_numeric() {
                write!(url, "&digits={digits}").ok()?;
            }

            let algorithm = match self.algorithm() {
                Algorithm::Unspecified => None,
                Algorithm::Sha1 => Some("SHA1"),
                Algorithm::Sha256 => Some("SHA256"),
                Algorithm::Sha512 => Some("SHA512"),
                Algorithm::Md5 => Some("MD5"),
            };

            if let Some(algorithm) = algorithm {
                write!(url, "&algorithm={algorithm}").ok()?;
            }

            Some(url)
        } else {
            None
        }
    }
}

pub fn print_info(payload: &MigrationPayload) -> std::io::Result<()> {
    let mut stdout = std::io::stdout().lock();

    writeln!(stdout, "Migration payload:")?;
    writeln!(stdout, "    Version: {}", payload.version)?;
    writeln!(stdout, "    Batch size: {}", payload.batch_size)?;
    writeln!(stdout, "    Batch index: {}", payload.batch_index)?;
    writeln!(stdout, "    Batch ID: {}", payload.batch_id)?;

    writeln!(
        stdout,
        "    Contained accounts ({}):",
        payload.otp_parameters.len()
    )?;
    for param in &payload.otp_parameters {
        writeln!(stdout, "    {}", param.name)?;
    }

    Ok(())
}

pub fn extract_tokens(payload: &MigrationPayload) -> std::io::Result<()> {
    let mut stdout = std::io::stdout().lock();

    for params in &payload.otp_parameters {
        writeln!(stdout, "{}:", params.name)?;
        writeln!(stdout, "    Issuer: {}", params.issuer)?;
        writeln!(
            stdout,
            "    Algorithm: {}",
            params.algorithm().as_str_name()
        )?;
        writeln!(stdout, "    Type: {}", params.r#type().as_str_name())?;

        if let Some(digits) = params.digits_numeric() {
            writeln!(stdout, "    Digits: {digits}")?;
        }

        writeln!(
            stdout,
            "    Secret (Base32): {}",
            data_encoding::BASE32.encode(&params.secret)
        )?;

        if params.r#type() == OtpType::Hotp {
            writeln!(stdout, "    Counter: {}", params.counter)?;
        }

        if let Some(otp_url) = params.otp_url() {
            writeln!(stdout, "    OTP URL: {otp_url}")?;
        }

        writeln!(stdout)?;
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum ExportTokensError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

fn get_export_key_value(args: &cli_args::ExportArgs, param: &OtpParameters) -> (String, String) {
    let key = &param.name;
    let value = match (args.secret_format, param.otp_url()) {
        (cli_args::SecretFormat::OtpUrl, Some(otp_url)) => otp_url,
        _ => data_encoding::BASE32.encode(&param.secret),
    };

    (key.into(), value)
}

pub fn export_tokens(
    payload: &MigrationPayload,
    args: &cli_args::ExportArgs,
) -> Result<(), ExportTokensError> {
    let otp_parameters: Box<dyn Iterator<Item = &OtpParameters>> = match args.token_types {
        cli_args::TokenTypes::Totp => Box::new(
            payload
                .otp_parameters
                .iter()
                .filter(|p| p.r#type() == OtpType::Totp),
        ),
        cli_args::TokenTypes::Hotp => Box::new(
            payload
                .otp_parameters
                .iter()
                .filter(|p| p.r#type() == OtpType::Hotp),
        ),
        cli_args::TokenTypes::All => Box::new(payload.otp_parameters.iter()),
    };

    let mut file: Box<dyn std::io::Write> =
        if args.output_file.to_str().is_some_and(|str| str == "-") {
            Box::new(std::io::stdout().lock())
        } else {
            Box::new(std::fs::File::create(&args.output_file)?)
        };

    if args.file_format.text {
        for param in otp_parameters {
            let (k, v) = get_export_key_value(args, param);
            writeln!(file, "{k}:{v}")?;
        }
    } else if args.file_format.json {
        let mut obj = serde_json::Map::new();
        for param in otp_parameters {
            let (k, v) = get_export_key_value(args, param);
            obj.insert(k, v.into());
        }

        if args.pretty_json {
            serde_json::to_writer_pretty(&mut file, &obj)?;
        } else {
            serde_json::to_writer(&mut file, &obj)?;
        }

        writeln!(file)?;
    };

    Ok(())
}
