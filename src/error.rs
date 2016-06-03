use std::error;
use std::fmt;
use std::io;
use std::result;
use std::string::FromUtf8Error;

use byteorder;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ByteOrder(byteorder::Error),
    UnknownType(String),
    Utf8(FromUtf8Error),
}

/// A short-hand for `result::Result<T, byteorder::Error>`.
pub type Result<T> = result::Result<T, Error>;

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => error::Error::description(err),
            Error::ByteOrder(ref err) => error::Error::description(err),
            Error::Utf8(ref err) => error::Error::description(err),
            Error::UnknownType(ref s) => &s,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => err.cause(),
            Error::ByteOrder(ref err) => err.cause(),
            Error::UnknownType(_) => None,
            Error::Utf8(ref err) => err.cause(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::ByteOrder(ref err) => err.fmt(f),
            Error::UnknownType(ref s) => write!(f, "unkown type {}", s),
            Error::Utf8(ref err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        Error::ByteOrder(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::Utf8(err)
    }
}
