use std::{error, fmt};

/// An error returned when a request times out.
#[derive(Debug, Default)]
pub struct TimeoutError(pub(super) ());

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "request timed out")
    }
}

impl error::Error for TimeoutError {}
