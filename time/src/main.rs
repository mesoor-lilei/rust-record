use std::env;

use chrono::Local;

use time::{parse_time, FORMAT};

fn main() {
    let mut args = env::args();
    if args.len() == 1 {
        let now = Local::now();
        println!("{}", now.format(FORMAT));
        println!("{}", now.timestamp_millis());
        return;
    }
    args.next();
    if args.len() == 1 {
        parse_time(args);
    }
}
