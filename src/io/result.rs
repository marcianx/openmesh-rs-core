//! Result type for mesh I/O operations.

extern crate byteorder;

use std::error;
use std::fmt;
use std::result;
use std::string;
use std::io;

/// OpenMesh I/O Error.
#[derive(Debug)]
pub enum Error {
    /// Unsupported functionality.
    Unsupported,
    /// Unexpected EOF.
    UnexpectedEOF,
    /// String exceeds 64Kb.
    StringExceeds64k,
    /// Invalid UTF-8.
    FromUtf8(string::FromUtf8Error),
    /// IO error.
    Io(io::Error)
}

impl fmt::Display for self::Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(match self {
            &Error::Io(ref err) => { err.fmt(formatter) }
            &Error::FromUtf8(ref err) => { err.fmt(formatter) }
            _ => { error::Error::description(self).fmt(formatter) }
        });
        result::Result::Ok(())
    }
}

impl error::Error for self::Error {
    fn description(&self) -> &str {
        match self {
            &Error::Unsupported => { "Unsupported functionality" }
            &Error::UnexpectedEOF => { "Unexpected EOF" }
            &Error::StringExceeds64k => { "Cannot store string longer than 64Kb" }
            &Error::Io(ref err) => { err.description() }
            &Error::FromUtf8(ref err) => { err.description() }
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &Error::Io(ref err)   => { err.cause() }
            &Error::FromUtf8(ref err) => { err.cause() }
            _ => { None }
        }
    }
}

/// OpenMesh I/O Result.
pub type Result<T> = result::Result<T, Error>;


impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::Io(err) }
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        match err {
            byteorder::Error::UnexpectedEOF => Error::UnexpectedEOF,
            byteorder::Error::Io(err) => Error::Io(err)
        }
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error { Error::FromUtf8(err) }
}
