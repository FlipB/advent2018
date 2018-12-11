use std;
use std::error;
use std::fmt;
use std::io;
use std::num;

#[derive(Debug)]
pub enum Error {
    IOErr(io::Error, String),
    ParseErr(num::ParseIntError, String),
    //StringErr(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // pass through to underlying implementation
        match *self {
            // To convert a String to a str you must slice it with '[..]' ? ok..
            Error::IOErr(ref err, _) => f.write_str(&(format!("{}", err)[..])),
            Error::ParseErr(ref err, _) => f.write_str(&(format!("{}", err)[..])),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        // pass through to underlying implementation
        match *self {
            Error::IOErr(ref err, _) => err.description(),
            // err.__description only available through the trait for this type
            Error::ParseErr(ref err, _) => error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        // pass through to underlying implementation
        match *self {
            Error::IOErr(ref err, _) => Some(err),
            Error::ParseErr(ref err, _) => Some(err),
        }
    }
}
