#[macro_use]
extern crate clap;

use std::error::Error;

use clap::{App, Arg, ArgMatches, SubCommand};
use crypto::aes::KeySize::KeySize128;
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};
use crypto::digest::Digest;
use crypto::md5::Md5;

fn matches() -> ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("md5").about("MD5 加密")
                .arg(Arg::with_name("text").help("明文").required(true)))
        .subcommand(
            SubCommand::with_name("aes").about("AES 加密")
                .args(&[
                    Arg::with_name("key").help("密钥").required(true),
                    Arg::with_name("text").help("明文").required(true)
                ])
        )
        .subcommand(
            SubCommand::with_name("unaes").about("AES 解密").
                args(&[
                    Arg::with_name("key").help("密钥").required(true),
                    Arg::with_name("text").help("密文").required(true)
                ])
        )
        .get_matches()
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = matches();
    if let Some(ref item) = matches.subcommand_matches("md5") {
        let mut md5 = Md5::new();
        md5.input_str(item.value_of("text").unwrap());
        println!("密文: {}", md5.result_str())
    } else if let Some(ref item) = matches.subcommand_matches("aes") {
        let key = item.value_of("key").unwrap().as_bytes();
        let text = item.value_of("text").unwrap().as_bytes();
        println!("密文: {}", hex::encode(encrypt(key, text)?));
    } else if let Some(ref item) = matches.subcommand_matches("unaes") {
        let key = item.value_of("key").unwrap().as_bytes();
        let text = &*hex::decode(item.value_of("text").unwrap())?;
        println!("明文: {}", String::from_utf8(decrypt(key, text)?)?);
    }
    Ok(())
}

/// 加密
/// 16, 24, or 32 字节的 key 对应 KeySize128, KeySize192, or KeySize256
fn encrypt(key: &[u8], text: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut encrypt = crypto::aes::ecb_encryptor(
        KeySize128,
        key,
        PkcsPadding,
    );
    let mut read_buffer = RefReadBuffer::new(text);
    let mut result = vec![0; text.len() * 4];
    let mut write_buffer = RefWriteBuffer::new(&mut result);
    encrypt.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
    Ok(result.into_iter().filter(|v| *v != 0).collect())
}

/// 解密
fn decrypt(key: &[u8], text: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut decrypt = crypto::aes::ecb_decryptor(
        KeySize128,
        key,
        PkcsPadding,
    );
    let mut read_buffer = RefReadBuffer::new(text);
    let mut result = vec![0; text.len()];
    let mut write_buffer = RefWriteBuffer::new(&mut result);
    decrypt.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
    Ok(result)
}
