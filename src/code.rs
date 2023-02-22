use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Code(pub HashMap<u8, usize>);

pub enum CodeFactory {
    Hex,
    Dec,
}

impl TryFrom<u8> for CodeFactory {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            10 => Ok(CodeFactory::Dec),
            16 => Ok(CodeFactory::Hex),
            _ => {
                return Err(format!(
                    "radixには[10, 16]のいずれかを指定してください。 r={}",
                    value
                ))
            }
        }
    }
}

impl CodeFactory {
    pub fn generate(&self, len: usize) -> Result<Code, String> {
        let mut rng = rand::thread_rng();
        let choices: Vec<u8> = (0..self.to_radix()).collect();

        let code = HashMap::from_iter(
            choices
                .choose_multiple(&mut rng, len)
                .cloned()
                .enumerate()
                .map(|(i, d)| (d, i)),
        );

        Ok(Code(code))
    }

    pub fn generate_from_string(&self, s: String) -> Result<Code, String> {
        let mut code = HashMap::new();

        for (i, c) in s.trim().chars().enumerate() {
            let d = match c.to_digit(self.to_radix() as u32) {
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

    fn to_radix(&self) -> u8 {
        match self {
            CodeFactory::Dec => 10,
            CodeFactory::Hex => 16,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::code::CodeFactory;

    use super::Code;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn generate() {
        let factory_dec = CodeFactory::try_from(10).unwrap();
        let factory_hex = CodeFactory::try_from(16).unwrap();

        let code = factory_dec.generate(4).unwrap();

        assert_eq!(code.0.len(), 4);

        let mut set = HashSet::new();

        for (d, i) in &code.0 {
            assert!(*d < 10);
            assert!(set.insert(*i));
        }

        let code = factory_dec.generate(8).unwrap();

        assert_eq!(code.0.len(), 8);

        let mut set = HashSet::new();

        for (d, i) in &code.0 {
            assert!(*d < 10);
            assert!(set.insert(*i));
        }

        let code = factory_hex.generate(8).unwrap();

        assert_eq!(code.0.len(), 8);

        let mut set = HashSet::new();

        for (d, i) in &code.0 {
            assert!(*d < 16);
            assert!(set.insert(*i));
        }
    }

    #[test]
    fn generate_from_string() {
        let factory_dec = CodeFactory::try_from(10).unwrap();
        let factory_hex = CodeFactory::try_from(16).unwrap();

        assert_eq!(
            factory_dec.generate_from_string("0123".to_string()),
            Ok(Code(HashMap::from([(0, 0), (1, 1), (2, 2), (3, 3)])))
        );

        assert_eq!(
            factory_dec.generate_from_string("0012".to_string()),
            Err("1つ目と2つ目の数字が重複しています。d=0".to_string())
        );

        assert_eq!(
            factory_dec.generate_from_string("01a3".to_string()),
            Err("数字として解釈できない文字があります。c=a".to_string())
        );

        assert_eq!(
            factory_hex.generate_from_string("01a3".to_string()),
            Ok(Code(HashMap::from([(0, 0), (1, 1), (0xa, 2), (3, 3)])))
        );
    }
}
