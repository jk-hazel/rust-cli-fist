use crate::{jwt_auth, jwt_verify, CmdExector};
use anyhow::Result;
use clap::{arg, command, Parser};
use serde::{Deserialize, Serialize};
use tracing::info;



#[derive(Debug, Parser)]
#[command(name = "jwt", about = "JWT operations")]
pub enum JwtSubCommand {
    #[command(about = "JWT sign")]
    Sign(JwtSignOpts),
    #[command(about = "JWT verify")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser, Clone)]
pub struct JwtSignOpts {
    #[arg(long, default_value = "sub_test")]
    pub sub: String,
    #[arg(short, long, default_value = "api://2132141")]
    pub aud: String,
    #[arg(short, long, default_value_t = 3600)]
    pub exp: i64,
    #[arg(short, long, default_value = "secret_test")]
    pub secret: String,
}

// jwt payload
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct JwtPayload {
    pub sub: String,
    pub aud: String,
    pub exp: i64,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, default_value = "-")]
    pub token: String,
    #[arg(short, long, default_value = "secret_test")]
    pub secret: String,
}

impl CmdExector for JwtSubCommand {
    async fn execute(self) -> Result<()> {
        match self {
            JwtSubCommand::Sign(opts) => opts.execute().await,
            JwtSubCommand::Verify(opts) => opts.execute().await,
        }
    }
}

impl CmdExector for JwtSignOpts {
    async fn execute(self) -> Result<()> {
        // create a JWTPayload struct by passing the sub, aud, and exp values
        let payload: JwtPayload = JwtPayload {
            sub: self.sub.clone(),
            aud: self.aud.clone(),
            exp: self.exp.clone(),
        };
        let token = jwt_auth(payload.clone(), self.secret).await?;
        info!("jwt sign use palyload:{:?}",payload);
        info!("jwt sign success:{}", token);
        Ok(())
    }
}

impl CmdExector for JwtVerifyOpts {
    async fn execute(self) -> Result<()> {
        let token_data = jwt_verify(self.token.clone(), self.secret.clone()).await?;
        info!("jwt verify success:{:?}", token_data);
        Ok(())
    }
}
