use std::fmt;

use super::{Code, Log};

pub struct Possibility(Vec<Code>);

impl From<Vec<Code>> for Possibility {
    fn from(value: Vec<Code>) -> Self {
        Self(value)
    }
}

impl Possibility {
    pub fn new() -> Self {
        vec![].into()
    }

    pub fn update(&mut self, log: &Log) {
        self.0.retain(|code| {
            if let Ok((result, _)) = code.diff(&log.guess) {
                result == log.result
            } else {
                false
            }
        });
    }
}

impl fmt::Display for Possibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("counts: {}\n", self.0.len());

        s += "possibilities: ";
        for code in self.0.get(0..5).unwrap_or(&self.0) {
            s += &format!("{}, ", code);
        }
        if self.0.len() > 5 {
            s += " etc...";
        }
        write!(f, "{}", s)
    }
}
