use clap::Parser;
use rcli::{CmdExector, Opts};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //这行代码的作用是初始化日志系统，这样我们就可以使用tracing宏来打印日志了
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    opts.cmd.execute().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
use rcli::{process_sign, text_verify, TextSignFormat};
    #[test]
fn test_process_sign() {
    let input = "-";
    let format = TextSignFormat::Blake3;
    let key = "fixtures/blake3.txt";
    process_sign(input, key, format).unwrap();
}

#[test]
fn test_text_verify() {
    let input = "-";
    let key = "fixtures/blake3.txt";
    let sig = "4lg4gzOpyJmh4yByGlpsWmlgKhEHUPOUMkqn3G7_Qa8";
    text_verify(input, key, sig).unwrap();
}
}

