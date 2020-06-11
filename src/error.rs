use std::error::Error;
use std::fmt;

/// A representation of simple errors constructed from strings
#[derive(Debug)]
pub(crate) struct StrErr(pub(crate) String);

impl Error for StrErr {}

impl fmt::Display for StrErr {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}