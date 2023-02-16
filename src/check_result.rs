use crate::code::Code;
use std::fmt;

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

        Ok(CheckResult {
            hit: hit,
            blow: blow,
        })
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
