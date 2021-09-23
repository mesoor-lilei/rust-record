use grep::{run, Config};
use std::{env, process::exit};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("参数异常 {}", err);
        exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("运行异常 {}", e);
        exit(1);
    };
}
