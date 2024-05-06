use std::str::FromStr;

use clap::{arg, Parser};

use crate::{process_csv, CmdExector};

use super::verify_file;


#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, value_parser = parser_format, default_value = "json")]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn parser_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl  From<OutputFormat> for &'static str{
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::JSON => "json".into(),
            OutputFormat::YAML => "yaml".into(),
            OutputFormat::TOML => "toml".into(),
        }
    }
    
}
impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::JSON),
            "yaml" => Ok(OutputFormat::YAML),
            "toml" => Ok(OutputFormat::TOML),
            _ => Err(anyhow::Error::msg(format!("Invalid format: {}", s))),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[derive(Debug,Clone, Copy)]
pub enum OutputFormat {
    JSON,
    YAML,
    TOML,
}

impl CmdExector for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output {
            output
        } else {
            format!("output.{}", self.format)
        };
        process_csv(&self.input, output, self.format)?;
        Ok(())
    }
}
