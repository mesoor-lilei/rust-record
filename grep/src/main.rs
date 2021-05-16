use std::{env, process};

use grep::{Config, run};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    println!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("参数异常 {}", err);
        process::exit(1)
    });
    if let Err(e) = run(config) {
        eprintln!("运行异常 {}", e);
        process::exit(1)
    };
}
