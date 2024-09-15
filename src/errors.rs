use std::ffi::OsString;
use std::fmt;
use std::io::{self};

#[derive(Debug)]
pub enum ThemeError {
    Io(io::Error),
    Utf8(OsString),
    InvalidChoice,
}

impl fmt::Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ThemeError::Io(ref error) => write!(f, "IO error: {}", error),
            ThemeError::Utf8(ref error) => write!(f, "UTF-8 error: {:?}", error),
            ThemeError::InvalidChoice => {
                write!(
                    f,
                    "Invalid theme choice. Please select a valid theme number."
                )
            }
        }
    }
}

impl From<io::Error> for ThemeError {
    fn from(error: io::Error) -> Self {
        ThemeError::Io(error)
    }
}

impl From<OsString> for ThemeError {
    fn from(error: OsString) -> Self {
        ThemeError::Utf8(error)
    }
}

impl From<std::num::ParseIntError> for ThemeError {
    fn from(_: std::num::ParseIntError) -> Self {
        ThemeError::InvalidChoice
    }
}
