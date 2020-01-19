use crate::ffi::EarwaxErrorCode;

use std::error::Error as StdError;
use std::ffi::NulError;

/// Specialized result.
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Nul(NulError),
    FFI(EarwaxErrorCode),
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        match *self {
            Error::Nul(ref e) => Some(e),
            _ => None,
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<NulError> for Error {
    fn from(error: NulError) -> Error {
        Error::Nul(error)
    }
}
