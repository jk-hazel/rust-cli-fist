mod process;
mod cli;
mod utils;
use enum_dispatch::enum_dispatch;
pub use process::*;
pub use cli::*;
use anyhow::Result;


#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> Result<()>;
}