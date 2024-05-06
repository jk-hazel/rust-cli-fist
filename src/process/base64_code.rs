
use anyhow::Ok;
use base64::{engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD}, Engine};
use std::{fs::File, io::Write};
use crate::{utils::*, Base64Format};


pub fn base64_encode(input: &str, format:Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}", encode);
    // 创建或打开文件
    let mut file = File::create("./fixtures/b64.txt")?;
    // 写入文件
    file.write_all(encode.as_bytes())?;
    Ok(())
}

pub fn base64_decode(input: &str,format:Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    println!("reader buf:{:?}", buf);
    let decode = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    let decode = String::from_utf8(decode)?;
    println!("decode buf:{}", decode);
    Ok(())
}



#[test]
fn base64_decode_test() {
    let input = "./fixtures/b64.txt";
    let format = Base64Format::Standard;
    base64_decode(input, format).unwrap();
}