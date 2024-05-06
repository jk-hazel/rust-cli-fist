
use std::{path::PathBuf, str::FromStr};
use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{process_http, CmdExector};



#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}


#[derive(Debug, Parser,Clone)]
pub struct HttpServeOpts {
    #[arg(short, long,default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExector for HttpServeOpts {
    async fn execute(self) -> Result<()> {
        process_http(self).await
    }
}

impl std::fmt::Display for HttpServeOpts{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "serve : --dir {} --port {}", self.dir.display(), self.port)
    }
}

impl FromStr for HttpSubCommand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "serve" => Ok(HttpSubCommand::Serve(HttpServeOpts::parse())),
            _ => Err(format!("Invalid subcommand: {}", s)),
        }
    }
}

impl std::fmt::Display for HttpSubCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpSubCommand::Serve(opts) => write!(f, "serve {}", opts),
        }
    }
    
}