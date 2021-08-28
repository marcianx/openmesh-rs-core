//! Result type for mesh I/O operations.

use std::error;
use std::fmt;
use std::io;
use std::result;
use std::string;

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
    Io(io::Error),
}

impl fmt::Display for self::Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        match *self {
            Error::Unsupported => "Unsupported functionality".fmt(f),
            Error::UnexpectedEOF => "Unexpected EOF".fmt(f),
            Error::StringExceeds64k => "Cannot store string longer than 64Kb".fmt(f),
            Error::Io(ref err) => err.fmt(f),
            Error::FromUtf8(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for self::Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::FromUtf8(ref err) => Some(err),
            _ => None,
        }
    }
}

/// OpenMesh I/O Result.
pub type Result<T> = result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        match err {
            byteorder::Error::UnexpectedEOF => Error::UnexpectedEOF,
            byteorder::Error::Io(err) => Error::Io(err),
        }
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::FromUtf8(err)
    }
}
