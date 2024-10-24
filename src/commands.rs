use crate::{authenticator_export::MigrationPayload, cli_args};
use std::fmt::Write;

pub fn print_info(payload: &MigrationPayload) {
    let contains_accounts = payload
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
        contains_accounts.trim_end()
    );
}

pub fn extract_tokens(payload: &MigrationPayload) {
    todo!()
}

#[derive(thiserror::Error, Debug)]
pub enum ExportTokensError {}

pub fn export_tokens(
    payload: &MigrationPayload,
    args: &cli_args::ExportArgs,
) -> Result<(), ExportTokensError> {
    todo!()
}
