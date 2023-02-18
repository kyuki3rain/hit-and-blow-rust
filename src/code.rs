use std::collections::HashMap;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

#[derive(Debug, PartialEq, Eq)]
pub struct Code(pub HashMap<u8, usize>);

impl Code {
    pub fn from_rand(rng: &mut ThreadRng) -> Self {
        let choices = (0..10).collect::<Vec<u8>>();

        Code(HashMap::from_iter(
            choices
                .choose_multiple(rng, 4)
                .cloned()
                .enumerate()
                .map(|(i, d)| (d, i)),
        ))
    }

    pub fn from_string(s: String) -> Result<Self, String> {
        let mut code = HashMap::new();

        for (i, c) in s.trim().chars().enumerate() {
            let d = match c.to_digit(10) {
                Some(d) => d,
                None => return Err(format!("数字として解釈できない文字があります。c={}", c)),
            };

            let res = code.insert(d as u8, i);
            if let Some(j) = res {
                return Err(format!(
                    "{}つ目と{}つ目の数字が重複しています。d={}",
                    j + 1,
                    i + 1,
                    d
                ));
            }
        }

        Ok(Code(code))
    }
}

#[cfg(test)]
mod tests {
    use super::Code;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn from_rand() {
        let mut rng = rand::thread_rng();
        let code = Code::from_rand(&mut rng);

        assert_eq!(code.0.len(), 4);

        let mut set = HashSet::new();

        for (d, i) in &code.0 {
            assert!(*d < 10);
            assert!(set.insert(*i));
        }
    }

    #[test]
    fn from_string() {
        assert_eq!(
            Code::from_string("0123".to_string()),
            Ok(Code(HashMap::from([(0, 0), (1, 1), (2, 2), (3, 3)])))
        );

        assert_eq!(
            Code::from_string("0012".to_string()),
            Err("1つ目と2つ目の数字が重複しています。d=0".to_string())
        );

        assert_eq!(
            Code::from_string("01a3".to_string()),
            Err("数字として解釈できない文字があります。c=a".to_string())
        );
    }
}
