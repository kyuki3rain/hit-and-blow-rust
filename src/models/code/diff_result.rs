use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct DiffResult {
    hit: usize,
    blow: usize,
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

    #[cfg(test)]
    pub fn create(hit: usize, blow: usize) -> Self {
        Self { hit, blow }
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
    fn hit() {
        let mut result = DiffResult::new();
        assert_eq!(!result.hit, 0);
        result.hit();
        assert_eq!(!result.hit, 1);
    }

    #[test]
    fn blow() {
        let mut result = DiffResult::new();
        assert_eq!(!result.blow, 0);
        result.blow();
        assert_eq!(!result.blow, 1);
    }

    #[test]
    fn correct() {
        let result = DiffResult::create(4, 0);
        assert!(result.correct(4));

        let result = DiffResult { hit: 1, blow: 0 };
        assert!(!result.correct(4));

        let result = DiffResult { hit: 8, blow: 0 };
        assert!(result.correct(8));
    }
}
