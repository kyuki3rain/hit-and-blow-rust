use crate::code::Code;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct CheckResult {
    hit: usize,
    blow: usize,
}

impl CheckResult {
    pub fn check(answer: &Code, guess: &Code) -> Result<Self, String> {
        let mut hit = 0;
        let mut blow = 0;

        if answer.0.len() != guess.0.len() {
            return Err(format!(
                "長さが間違っています。ans={}, guess={}",
                answer.0.len(),
                guess.0.len()
            ));
        }

        for (val, i) in guess.0.iter() {
            if let Some(j) = answer.0.get(val) {
                if i == j {
                    hit += 1;
                } else {
                    blow += 1;
                }
            }
        }

        Ok(CheckResult { hit, blow })
    }

    pub fn correct(&self, len: usize) -> bool {
        self.hit == len && self.blow == 0
    }
}

impl fmt::Display for CheckResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Result: {}hit, {}blow", self.hit, self.blow)
    }
}

#[cfg(test)]
mod tests {
    use super::CheckResult;
    use crate::code::CodeFactory;

    #[test]
    fn check() {
        let factory = CodeFactory::try_from(10).unwrap();

        let answer = factory.generate_from_string("0123".to_string()).unwrap();

        let guess = factory.generate_from_string("0123".to_string()).unwrap();
        assert_eq!(
            CheckResult::check(&answer, &guess),
            Ok(CheckResult { hit: 4, blow: 0 })
        );

        let guess = factory.generate_from_string("0369".to_string()).unwrap();
        assert_eq!(
            CheckResult::check(&answer, &guess),
            Ok(CheckResult { hit: 1, blow: 1 })
        );

        let guess = factory.generate_from_string("4567".to_string()).unwrap();
        assert_eq!(
            CheckResult::check(&answer, &guess),
            Ok(CheckResult { hit: 0, blow: 0 })
        );

        let guess = factory.generate_from_string("01234".to_string()).unwrap();
        assert_eq!(
            CheckResult::check(&answer, &guess),
            Err("長さが間違っています。ans=4, guess=5".to_string())
        );

        let answer = factory
            .generate_from_string("01234567".to_string())
            .unwrap();
        let guess = factory
            .generate_from_string("01234567".to_string())
            .unwrap();
        assert_eq!(
            CheckResult::check(&answer, &guess),
            Ok(CheckResult { hit: 8, blow: 0 })
        );
    }

    #[test]
    fn correct() {
        let result = CheckResult { hit: 4, blow: 0 };
        assert!(result.correct(4));

        let result = CheckResult { hit: 1, blow: 0 };
        assert!(!result.correct(4));

        let result = CheckResult { hit: 8, blow: 0 };
        assert!(result.correct(8));
    }
}
