use std::env::Args;

use chrono::{Local, TimeZone};

pub const FORMAT: &str = "%F %H:%M:%S%.3f";

pub fn parse_time(mut args: Args) {
    let date = match args.next() {
        Some(arg) => {
            match arg.parse::<i64>() {
                // 可以转为 i64 则为时间戳
                Ok(arg_value) => Local.timestamp_millis(arg_value).format(FORMAT).to_string(),
                Err(_) => match Local.datetime_from_str(&arg, FORMAT) {
                    Ok(nav_date) => nav_date.timestamp_millis().to_string(),
                    Err(e) => {
                        eprintln!("参数 date 解析失败 {}", e);
                        return;
                    }
                },
            }
        }
        None => {
            eprintln!("参数 date 不存在");
            return;
        }
    };
    println!("{}", date);
}

#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone};

    use crate::FORMAT;

    #[test]
    fn test() {
        let time_str = "2020-02-20 02:04:08.000";
        let timestamp = 1582135448000;
        assert_eq!(
            time_str,
            Local.timestamp_millis(timestamp).format(FORMAT).to_string()
        );
        assert_eq!(
            timestamp,
            Local
                .datetime_from_str(time_str, FORMAT)
                .unwrap()
                .timestamp_millis()
        );
    }
}
