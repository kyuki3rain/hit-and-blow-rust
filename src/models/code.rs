pub mod diff_result;
pub use diff_result::DiffResult;

use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Code(pub HashMap<u8, usize>);

impl Code {
    pub fn from_size(len: usize, radix: u32) -> Self {
        let mut rng = rand::thread_rng();
        let choices: Vec<u8> = (0..(radix as u8)).collect();

        let code = HashMap::from_iter(
            choices
                .choose_multiple(&mut rng, len)
                .cloned()
                .enumerate()
                .map(|(i, d)| (d, i)),
        );

        Self(code)
    }

    pub fn from_string(s: String, radix: u32) -> Result<Self, String> {
        let mut code = HashMap::new();

        for (i, c) in s.trim().chars().enumerate() {
            let d = match c.to_digit(radix) {
                Some(d) => d as u8,
                None => return Err(format!("数字として解釈できない文字があります。c={}", c)),
            };

            if let Some(j) = code.insert(d, i) {
                return Err(format!(
                    "{}つ目と{}つ目の数字が重複しています。d={}",
                    j + 1,
                    i + 1,
                    d
                ));
            }
        }

        Ok(Self(code))
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
    use std::collections::{HashMap, HashSet};

    use super::Code;
    use super::DiffResult;

    #[test]
    fn from_size() {
        let code = Code::from_size(4, 10);

        assert_eq!(code.0.len(), 4);

        let mut set = HashSet::new();

        for (d, i) in &code.0 {
            assert!(*d < 10);
            assert!(set.insert(*i));
        }

        let code = Code::from_size(8, 10);

        assert_eq!(code.0.len(), 8);

        let mut set = HashSet::new();

        for (d, i) in &code.0 {
            assert!(*d < 10);
            assert!(set.insert(*i));
        }

        let code = Code::from_size(8, 16);

        assert_eq!(code.0.len(), 8);

        let mut set = HashSet::new();

        for (d, i) in &code.0 {
            assert!(*d < 16);
            assert!(set.insert(*i));
        }
    }

    #[test]
    fn generate_from_string() {
        assert_eq!(
            Code::from_string("0123".to_string(), 10),
            Ok(Code(HashMap::from([(0, 0), (1, 1), (2, 2), (3, 3)])))
        );

        assert_eq!(
            Code::from_string("0012".to_string(), 10),
            Err("1つ目と2つ目の数字が重複しています。d=0".to_string())
        );

        assert_eq!(
            Code::from_string("01a3".to_string(), 10),
            Err("数字として解釈できない文字があります。c=a".to_string())
        );

        assert_eq!(
            Code::from_string("01a3".to_string(), 16),
            Ok(Code(HashMap::from([(0, 0), (1, 1), (0xa, 2), (3, 3)])))
        );
    }

    #[test]
    fn diff() {
        let answer = Code::from_string("0123".to_string(), 10).unwrap();

        let guess = Code::from_string("0123".to_string(), 10).unwrap();
        assert_eq!(answer.diff(&guess), Ok(DiffResult { hit: 4, blow: 0 }));

        let guess = Code::from_string("0369".to_string(), 10).unwrap();
        assert_eq!(answer.diff(&guess), Ok(DiffResult { hit: 1, blow: 1 }));

        let guess = Code::from_string("4567".to_string(), 10).unwrap();
        assert_eq!(answer.diff(&guess), Ok(DiffResult { hit: 0, blow: 0 }));

        let guess = Code::from_string("01234".to_string(), 10).unwrap();
        assert_eq!(
            answer.diff(&guess),
            Err("長さが間違っています。ans=4, guess=5".to_string())
        );

        let answer = Code::from_string("01234567".to_string(), 10).unwrap();
        let guess = Code::from_string("01234567".to_string(), 10).unwrap();
        assert_eq!(answer.diff(&guess), Ok(DiffResult { hit: 8, blow: 0 }));
    }
}
