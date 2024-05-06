pub use std::path::Path;
pub use clap::{command, Parser};
use enum_dispatch::enum_dispatch;
pub use self::{csv_cli::*, genpass_cli::*, base64::*, text::*, http_cli::*, jwt_cli::*};
mod csv_cli;
mod genpass_cli;
mod base64;
mod text;
mod http_cli;
mod jwt_cli;



#[derive(Debug, Parser)]
#[command(name = "csv2json", about = "Convert CSV to JSON", long_about = None, version, author)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    CSV(CsvOpts),
    #[command(name = "genpass", about = "Generate password")]
    Genpass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
    #[command(subcommand)]
    Jwt(JwtSubCommand),
} 



/// Verify that the input base64 file exists
pub fn verify_file(file_name: &str) -> Result<String, String> {
    //if input is "-" or file exists
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err(format!("File not found: {}", file_name))
    }
}

pub fn verify_dir(dir: &str) -> Result<String, String> {
    if Path::new(dir).is_dir() {
        Ok(dir.into())
    } else {
        Err(format!("Directory not found: {}", dir))
    }
}
// Verify that the input Expiration format
pub fn verify_exp_format(exp: &str) -> Result<String, String> {
    if exp.parse::<i64>().is_ok() {
        Ok(exp.into())
    } else {
        Err(format!("Invalid exp format: {}", exp))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.tomlcargo".into()));
        assert_eq!(verify_file("not-exist"), Err("File not found: not-exist".into()));
    }

}


// Verify that the input base64 file format
pub fn parse_base64_format(format: &str) -> Result<Base64Format, String> {
    match format {
        "standard" => Ok(Base64Format::Standard),
        "urlsafe" => Ok(Base64Format::UrlSafe),
        _ => Err(format!("Invalid format: {}", format)),
    }
}
