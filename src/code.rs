use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Code(pub HashMap<u8, usize>, u8);

impl Code {
    pub fn from_rand(len: usize, radix: u8) -> Result<Self, String> {
        let radix = match radix {
            10 | 16 => radix,
            radix => {
                return Err(format!(
                    "radixには[10, 16]のいずれかを指定してください。 r={}",
                    radix
                ))
            }
        };

        if len > radix as usize {
            return Err(format!(
                "長さは{}以下である必要があります。l={}",
                radix, len
            ));
        }

        let mut rng = rand::thread_rng();
        let choices: Vec<u8> = (0..radix).collect();

        let code = HashMap::from_iter(
            choices
                .choose_multiple(&mut rng, len)
                .cloned()
                .enumerate()
                .map(|(i, d)| (d, i)),
        );

        Ok(Code(code, radix))
    }

    pub fn from_string(s: String, radix: u8) -> Result<Self, String> {
        let radix = match radix {
            10 | 16 => radix,
            radix => {
                return Err(format!(
                    "radixには[10, 16]のいずれかを指定してください。 r={}",
                    radix
                ))
            }
        };

        let mut code = HashMap::new();

        for (i, c) in s.trim().chars().enumerate() {
            let d = match c.to_digit(radix as u32) {
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

        Ok(Code(code, radix))
    }
}

#[cfg(test)]
mod tests {
    use super::Code;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn from_rand() {
        let code = Code::from_rand(4, 10).unwrap();

        assert_eq!(code.0.len(), 4);

        let mut set = HashSet::new();

        for (d, i) in &code.0 {
            assert!(*d < 10);
            assert!(set.insert(*i));
        }

        let code = Code::from_rand(8, 10).unwrap();

        assert_eq!(code.0.len(), 8);

        let mut set = HashSet::new();

        for (d, i) in &code.0 {
            assert!(*d < 10);
            assert!(set.insert(*i));
        }

        let code = Code::from_rand(8, 16).unwrap();

        assert_eq!(code.0.len(), 8);

        let mut set = HashSet::new();

        for (d, i) in &code.0 {
            assert!(*d < 16);
            assert!(set.insert(*i));
        }
    }

    #[test]
    fn from_string() {
        assert_eq!(
            Code::from_string("0123".to_string(), 10),
            Ok(Code(HashMap::from([(0, 0), (1, 1), (2, 2), (3, 3)]), 10))
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
            Ok(Code(HashMap::from([(0, 0), (1, 1), (0xa, 2), (3, 3)]), 16))
        );
    }
}
