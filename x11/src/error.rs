//! No-op library for API compatibility with dynamically loaded version. The compile-time version
//! will never throw an OpenError.

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct OpenError;

impl Display for OpenError {
    fn fmt(&self, _f: &mut Formatter) -> Result<(), ::std::fmt::Error> {
        Ok(())
    }
}

impl Error for OpenError {
    fn description(&self) -> &str {
        "unused"
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OpenErrorKind {
    Unused,
}

impl OpenErrorKind {
    pub fn as_str(self) -> &'static str {
        match self {
            OpenErrorKind::Unused => "unused",
        }
    }
}
