use std::env::Args;
use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.file)?;
    let result = if config.case_sensitive {
        search(&config.query, &text)
    } else {
        search_case_insensitive(&config.query, &text)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Self, &'static str> {
        println!("{:?}", args);
        if args.len() < 3 {
            return Err("参数小于两位");
        }
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("参数 query 不存在"),
        };
        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("参数 file 不存在"),
        };
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        Ok(Self {
            query,
            file,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    text.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    text.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{search, search_case_insensitive};

    #[test]
    fn case_sensitive() {
        let query = "one";
        let text = "\
one
two
three";
        assert_eq!(vec!["one"], search(query, text));
    }

    #[test]
    fn case_insensitive() {
        let query = "one";
        let text = "\
ONE
two
three";
        assert_eq!(vec!["ONE"], search_case_insensitive(query, text));
    }
}
