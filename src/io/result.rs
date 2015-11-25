extern crate byteorder;

use std::error;
use std::fmt;
use std::result;
use std::io;

/// OpenMesh I/O Error.
#[derive(Debug)]
pub enum Error {
    Unsupported,
    UnexpectedEOF,
    Io(io::Error)
}

impl fmt::Display for self::Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(match self {
            &Error::Unsupported   => { "Unsupported functionality".fmt(formatter) }
            &Error::UnexpectedEOF => { "Unexpected EOF".fmt(formatter) }
            &Error::Io(ref err)   => { err.fmt(formatter) }
        });
        result::Result::Ok(())
    }
}

impl error::Error for self::Error {
    fn description(&self) -> &str {
        match self {
            &Error::Unsupported => { "Unsupported functionality" }
            &Error::UnexpectedEOF => { "Unexpected EOF" }
            &Error::Io(ref err) => { err.description() }
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &Error::Unsupported   => { None }
            &Error::UnexpectedEOF => { None }
            &Error::Io(ref err)   => { err.cause() }
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
