use clap::{arg, Parser};
use crate::{base64_decode, base64_encode, CmdExector};
use super::{parse_base64_format,verify_file};

#[derive(Debug, Parser)]
pub struct Base64Opts {
    #[arg(short,long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long,value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Base64 encode")]
    Encode(Base64Opts),
    #[command(name = "decode", about = "Base64 decode")]
    Decode(Base64Opts),
}


#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl std::fmt::Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "standard"),
            Base64Format::UrlSafe => write!(f, "urlsafe"),
        }
    }
}

impl std::str::FromStr for Base64Format {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
    
}


impl CmdExector for Base64SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => {
                base64_encode(&opts.input, opts.format)
            }
            Base64SubCommand::Decode(opts) => {
                base64_decode(&opts.input, opts.format)
            }
        }
    }
}


