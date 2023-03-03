use std::fmt;

use super::{Code, DiffResult};

#[derive(Debug)]
pub struct Log {
    pub guess: Code,
    pub result: DiffResult,
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.result)
    }
}
