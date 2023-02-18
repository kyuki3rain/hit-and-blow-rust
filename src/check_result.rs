use crate::code::Code;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct CheckResult {
    hit: u8,
    blow: u8,
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

        for (idx, &val) in guess.0.iter().enumerate() {
            if let Some(ans_idx) = answer.0.iter().position(|&ans| ans == val) {
                if ans_idx == idx {
                    hit += 1;
                } else {
                    blow += 1;
                }
            }
        }

        Ok(CheckResult { hit, blow })
    }

    pub fn correct(&self) -> bool {
        self.hit == 4 && self.blow == 0
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
    use super::Code;

    #[test]
    fn check() {
        let answer = Code::from_string("0123".to_string()).unwrap();

        let guess = Code::from_string("0123".to_string()).unwrap();
        assert_eq!(
            CheckResult::check(&answer, &guess),
            Ok(CheckResult { hit: 4, blow: 0 })
        );

        let guess = Code::from_string("0369".to_string()).unwrap();
        assert_eq!(
            CheckResult::check(&answer, &guess),
            Ok(CheckResult { hit: 1, blow: 1 })
        );

        let guess = Code::from_string("4567".to_string()).unwrap();
        assert_eq!(
            CheckResult::check(&answer, &guess),
            Ok(CheckResult { hit: 0, blow: 0 })
        );

        let guess = Code::from_string("01234".to_string()).unwrap();
        assert_eq!(
            CheckResult::check(&answer, &guess),
            Err("長さが間違っています。ans=4, guess=5".to_string())
        );
    }

    #[test]
    fn correct() {
        let result = CheckResult { hit: 4, blow: 0 };
        assert!(result.correct());

        let result = CheckResult { hit: 1, blow: 0 };
        assert!(!result.correct());
    }
}
