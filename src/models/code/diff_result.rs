use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct DiffResult {
    pub hit: usize,
    pub blow: usize,
}

impl DiffResult {
    pub fn new() -> Self {
        Self { hit: 0, blow: 0 }
    }

    pub fn hit(&mut self) {
        self.hit += 1;
    }

    pub fn blow(&mut self) {
        self.blow += 1;
    }

    pub fn correct(&self, len: usize) -> bool {
        self.hit == len && self.blow == 0
    }
}

impl fmt::Display for DiffResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Result: {}hit, {}blow", self.hit, self.blow)
    }
}

#[cfg(test)]
mod tests {
    use super::DiffResult;

    #[test]
    fn correct() {
        let result = DiffResult { hit: 4, blow: 0 };
        assert!(result.correct(4));

        let result = DiffResult { hit: 1, blow: 0 };
        assert!(!result.correct(4));

        let result = DiffResult { hit: 8, blow: 0 };
        assert!(result.correct(8));
    }
}
