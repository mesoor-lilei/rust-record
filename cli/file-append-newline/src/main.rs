use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};

use once_cell::sync::Lazy;

mod config;

type Result<T = ()> = anyhow::Result<T>;

static CONFIG: Lazy<config::Config> = Lazy::new(config::read_config);

fn main() -> Result {
    recursion(&CONFIG.path)?;
    Ok(())
}

fn recursion(path: &str) -> Result {
    // 排除前缀
    for exclude_prefix in &CONFIG.exclude_prefix {
        if path.starts_with(&(CONFIG.path.to_string() + "/" + exclude_prefix)) {
            return Ok(());
        }
    }

    // 排除后缀
    for exclude_suffix in &CONFIG.exclude_suffix {
        if path.ends_with(exclude_suffix) {
            return Ok(());
        }
    }

    let file_type = fs::metadata(path)?.file_type();
    let is_file = file_type.is_file();
    let is_dir = file_type.is_dir();

    // 递归目录
    if is_dir {
        for item in fs::read_dir(path)? {
            recursion(item?.path().to_str().unwrap())?;
        }
    } else if is_file {
        let mut file = OpenOptions::new().read(true).append(true).open(path)?;
        let mut str = String::new();
        file.read_to_string(&mut str)?;
        print!("{}", path);
        if let Some(last_char) = str.chars().last() {
            if last_char == '\n' {
                println!(" 已存在换行符");
            } else {
                writeln!(file)?;
                println!(" 已追加换行符");
            }
        } else {
            println!(" 空文件");
        }
    }
    Ok(())
}
