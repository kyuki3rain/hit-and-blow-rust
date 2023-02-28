use std::fmt;

use super::Code;

pub struct Possibility(Vec<Code>);

impl IntoIterator for Possibility {
    type Item = Code;
    type IntoIter = std::vec::IntoIter<Code>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<Vec<Code>> for Possibility {
    fn from(value: Vec<Code>) -> Self {
        Self(value)
    }
}

impl Possibility {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, code: Code) {
        self.0.push(code)
    }
}

impl fmt::Display for Possibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("counts: {}\n", self.0.len());

        s += &format!("possibilities: ");
        for code in self.0.get(0..5).unwrap_or(&self.0) {
            s += &format!("{}, ", code);
        }
        if self.0.len() > 5 {
            s += &format!(" etc...");
        }
        write!(f, "{}", s)
    }
}
