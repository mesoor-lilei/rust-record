use std::fmt::Formatter;
use std::{error, fmt};

use crate::Kind::Message;

#[derive(Debug)]
enum Kind {
    Message(&'static str),
}

#[derive(Debug)]
pub struct Error(Kind);

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let out = match &self.0 {
            Message(v) => v,
        };
        write!(f, "message -> {}", out)
    }
}

impl From<Kind> for Error {
    fn from(k: Kind) -> Self {
        Error(k)
    }
}

impl Error {
    pub fn message(str: &'static str) -> Self {
        Error(Message(str))
    }
}

#[test]
fn test() -> Result<(), Error> {
    Err(Error::message("错误信息"))
}
