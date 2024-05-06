
use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{process_sign, text_verify, verify_file, CmdExector};



#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum TextSubCommand {
    #[command(about = "sign a message with a private key")]
    Sign(TextSignOpts),
    #[command(about = "verify a message with a public key")]
    Verify(TextVerifyOpts),
    
}


#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short,long, default_value = "-",value_parser = verify_file)]
    pub input: String,
    #[arg(short, value_parser = verify_file, default_value = "-")]
    pub key: String,
    #[arg(long, default_value_t = TextSignFormat::Blake3, value_parser = parse_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short,long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short,long, value_parser = verify_file, default_value = "-")]
    pub key: String,
    #[arg(short,long)]
    pub sig: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_sign_format(format: &str) -> Result<TextSignFormat, String> {
    format.parse()
}

impl std::str::FromStr for TextSignFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
    
}

impl std::fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextSignFormat::Blake3 => write!(f, "blake3"),
            TextSignFormat::Ed25519 => write!(f, "ed25519"),
        }
    }
}

impl CmdExector for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_sign(&self.input, &self.key, self.format)
    }
}

impl CmdExector for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        text_verify(&self.input, &self.key, &self.sig)?;
        Ok(())
    }
    
}