use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Code(pub HashMap<u8, usize>);

impl Code {
    pub fn from_rand(len: usize) -> Result<Self, String> {
        if len > 10 {
            return Err(format!("長さは10以下である必要があります。l={}", len));
        }

        let mut rng = rand::thread_rng();
        let choices: Vec<u8> = (0..10).collect();

        Ok(Code(HashMap::from_iter(
            choices
                .choose_multiple(&mut rng, len)
                .cloned()
                .enumerate()
                .map(|(i, d)| (d, i)),
        )))
    }

    pub fn from_string(s: String) -> Result<Self, String> {
        let mut code = HashMap::new();

        for (i, c) in s.trim().chars().enumerate() {
            let d = match c.to_digit(10) {
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

        Ok(Code(code))
    }
}

#[cfg(test)]
mod tests {
    use super::Code;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn from_rand() {
        let code = Code::from_rand(4).unwrap();

        assert_eq!(code.0.len(), 4);

        let mut set = HashSet::new();

        for (d, i) in &code.0 {
            assert!(*d < 10);
            assert!(set.insert(*i));
        }

        let code = Code::from_rand(8).unwrap();

        assert_eq!(code.0.len(), 8);

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
