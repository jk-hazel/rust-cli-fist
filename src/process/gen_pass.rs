use std::{fs::File, io::Write};

use anyhow::Ok;
use rand::seq::IteratorRandom;
use zxcvbn::zxcvbn;

use crate::cli::GenPassOpts; // Import the zxcvbn crate

pub fn gen_pass(opts: GenPassOpts) -> anyhow::Result<(),anyhow::Error>  {
    let mut pass_vec = Vec::new();
    let mut pass = String::new();
        if opts.uppercase {
            pass.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if opts.number {
            pass.push_str("0123456789");
        }
        if opts.symbol {
            pass.push_str("!@#$%&*_+-=");
        }
        let pass: String = pass.chars().collect();
        let pass: String = pass.chars().collect();
        pass_vec.push(pass);   
        println!("genpass_length:{}", opts.length);
        let mut password = String::new();
        for _ in 0..opts.length {
            let pass: String = pass_vec.iter().map(|x| x.chars().choose(&mut rand::thread_rng()).unwrap()).collect::<String>();
            password.push_str(&pass);
        }
        let esimat = zxcvbn(&password, &[])?;
        eprintln!("Password strength: {}", esimat.score());
        println!("Password: {} \n", password);
        // 创建或打开文件
        let mut file = File::create("./fixtures/blake3.txt")?;
        // 写入文件
        file.write_all(password.as_bytes())?;
    Ok(())
}