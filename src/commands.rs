use base64::Engine;

use crate::{
    authenticator_export::{migration_payload::OtpType, MigrationPayload},
    cli_args,
};
use std::io::Write;

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

        let digits = match params.digits() {
            crate::authenticator_export::migration_payload::DigitCount::Unspecified => "N/A",
            crate::authenticator_export::migration_payload::DigitCount::Six => "6",
            crate::authenticator_export::migration_payload::DigitCount::Eight => "8",
        };

        writeln!(stdout, "    Digits: {digits}")?;

        writeln!(
            stdout,
            "    Secret: {}",
            base64::prelude::BASE64_STANDARD.encode(&params.secret)
        )?;

        if params.r#type() == OtpType::Hotp {
            writeln!(stdout, "    Counter: {}", params.counter)?;
        }

        writeln!(stdout)?;
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum ExportTokensError {}

pub fn export_tokens(
    payload: &MigrationPayload,
    args: &cli_args::ExportArgs,
) -> Result<(), ExportTokensError> {
    todo!()
}
