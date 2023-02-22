use super::Code;

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
    pub fn generate(&self, len: usize) -> Code {
        Code::from_size(len, self.to_radix())
    }

    pub fn generate_from_string(&self, s: String) -> Result<Code, String> {
        Code::from_string(s, self.to_radix())
    }

    fn to_radix(&self) -> u32 {
        match self {
            CodeFactory::Dec => 10,
            CodeFactory::Hex => 16,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::code::{code_factory::CodeFactory, Code};

    #[test]
    fn generate() {
        let factory_dec = CodeFactory::try_from(10).unwrap();
        let factory_hex = CodeFactory::try_from(16).unwrap();

        let code = factory_dec.generate(4);

        assert_eq!(code.0.len(), 4);

        let mut set = HashSet::new();

        for (d, i) in &code.0 {
            assert!(*d < 10);
            assert!(set.insert(*i));
        }

        let code = factory_dec.generate(8);

        assert_eq!(code.0.len(), 8);

        let mut set = HashSet::new();

        for (d, i) in &code.0 {
            assert!(*d < 10);
            assert!(set.insert(*i));
        }

        let code = factory_hex.generate(8);

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
