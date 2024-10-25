# google-authenticator-export

Google Authenticator is a widely used mobile app used to generate OTP tokens for 2FA logins.
This utility is made for when you inevitably want to switch your OTP tokens over to an open-source solution [and](https://github.com/bitwarden/) / or your password manager.
In order to not have to go through all the trouble of going through the 2FA setup a second time, the app offers an export via QR code, usually meant for tranferring over to a new device.
Using this tool however, you can extract the raw OTP secrets from the QR code and export them to a variety of formats.

## Downloading

    cargo install --git https://github.com/lennyerik/google-authenticator-export.git

## Basic Usage

In the mobile app, go to Menu->Transfer Accounts->Export Accounts, take a screenshot (or a picture of your screen, you monster) of the QR code and transfer it to your computer.
Now, all you need to do is to run:

    google-authenticator-export <path_to_qr_code> info

to see what accounts are contained in the QR code and then run

    google-authenticator-export <path_to_qr_code> extract

in order to get a human-readable list of all the tokens.

> [!TIP]
> If you do not have a picture of an exported QR handy, feel free to try the commands with `src/test/img/demo_export.png`

## Exporting

If you have a *lot* of OTP codes and prefer automating the workflow of importing them into your new application of choice, consider the `export` subcommand instead.

From `google-authenticator-export help export`:

```
Export the OTP secrets to different file formats

Usage: google-authenticator-export <IMAGE_PATH> export [OPTIONS] <--json|--text>

Options:
  -j, --json
          Export in JSON format

  -t, --text
          Export in text format

  -o, --output-file <OUTPUT_FILE>
          The file to output to. If unset or set to '-' the exported data will be printed to stdout
          
          [default: -]

  -s, --secret-format <SECRET_FORMAT>
          [default: base32]

          Possible values:
          - base32:  Export the base32 secret only
          - otp-url: Export the full OTP URL (where available)

  -t, --token-types <TOKEN_TYPES>
          Which token types to export
          
          [default: totp]
          [possible values: totp, hotp, all]

      --pretty-json
          Prettify JSON output

  -h, --help
          Print help (see a summary with '-h')
```
