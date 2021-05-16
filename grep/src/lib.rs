use std::{env, fs};
use std::error::Error;

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
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("参数小于两位");
        }
        let query = args[1].clone();
        let file = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        Ok(Self { query, file, case_sensitive })
    }
}

pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in text.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in text.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn case_sensitive() {
        let query = "one";
        let text = "\
one
two
three";
        assert_eq!(vec!["one"], search(query, text))
    }

    #[test]
    fn case_insensitive() {
        let query = "one";
        let text = "\
ONE
two
three";
        assert_eq!(vec![], search(query, text))
    }
}
