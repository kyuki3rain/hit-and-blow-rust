pub mod diff_result;
pub use diff_result::DiffResult;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Code(HashMap<u8, usize>);

impl Code {
    pub fn new(init: HashMap<u8, usize>) -> Self {
        Self(init)
    }

    #[cfg(test)]
    pub fn code(&self) -> &HashMap<u8, usize> {
        &self.0
    }

    pub fn diff(&self, guess: &Code) -> Result<DiffResult, String> {
        if self.0.len() != guess.0.len() {
            return Err(format!(
                "長さが間違っています。ans={}, guess={}",
                self.0.len(),
                guess.0.len()
            ));
        }

        let mut result = DiffResult::new();

        for (val, i) in guess.0.iter() {
            if let Some(j) = self.0.get(val) {
                if i == j {
                    result.hit();
                } else {
                    result.blow();
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{factories::CodeFactory, models::code::DiffResult};

    #[test]
    fn diff() {
        let factory = CodeFactory::Dec;

        let answer = factory.generate_from_str("0123").unwrap();

        let guess = factory.generate_from_str("0123").unwrap();
        assert_eq!(answer.diff(&guess), Ok(DiffResult { hit: 4, blow: 0 }));

        let guess = factory.generate_from_str("0369").unwrap();
        assert_eq!(answer.diff(&guess), Ok(DiffResult { hit: 1, blow: 1 }));

        let guess = factory.generate_from_str("4567").unwrap();
        assert_eq!(answer.diff(&guess), Ok(DiffResult { hit: 0, blow: 0 }));

        let guess = factory.generate_from_str("01234").unwrap();
        assert_eq!(
            answer.diff(&guess),
            Err("長さが間違っています。ans=4, guess=5".to_string())
        );

        let answer = factory.generate_from_str("01234567").unwrap();
        let guess = factory.generate_from_str("01234567").unwrap();
        assert_eq!(answer.diff(&guess), Ok(DiffResult { hit: 8, blow: 0 }));
    }
}
