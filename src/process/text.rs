use std::{fs, io::Read, path::Path};
use anyhow::Ok;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

use crate::{utils::get_reader, TextSignFormat};
pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, input: impl Read, sig: &[u8]) -> anyhow::Result<bool>;
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Blake3Key {
    key: [u8; 32],
}

impl Blake3Key {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let hasher = Blake3Key::new(key);
        Ok(hasher)
    }
    pub fn load_key(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Ed25519Key {
    key: [u8; 32],
}

impl TextSign for Blake3Key {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes().to_vec())
    }
}

impl TextVerify for Blake3Key{
    fn verify(&self, mut input: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        let hash: blake3::Hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes() == sig)
    }
    
}

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<()> {
    let mut buf = get_reader(input)?;
    let sign = match format {
        TextSignFormat::Blake3 => {
            let hasher = Blake3Key::load_key(key)?;
            hasher.sign(&mut buf)?
        }
        TextSignFormat::Ed25519 => todo!("Ed25519")
    };
    let sig = URL_SAFE_NO_PAD.encode(&sign);
    println!("get sig :{}", sig);
    Ok(())
}

pub fn text_verify(input: &str, key: &str, signature: &str) -> anyhow::Result<bool> {
    let key = fs::read(key)?;
    let key: &[u8] = &key[..32];
    let key = key.try_into()?;
    let hasher = Blake3Key{ key};
    let sig = URL_SAFE_NO_PAD.decode(signature.as_bytes())?;
    let mut buf = get_reader(input)?;
    let result = hasher.verify(&mut buf, &sig)?;
    if result {
        println!("Signature:{signature} is valid");
    } else {
        println!("Signature:{signature} is invalid");
    }
    Ok(result)
}