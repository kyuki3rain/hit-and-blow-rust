use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

#[derive(Debug, PartialEq, Eq)]
pub struct Code(pub Vec<u8>);

impl Code {
    pub fn from_rand(rng: &mut ThreadRng) -> Self {
        let choices = (0..10).collect::<Vec<u8>>();

        Code(choices.choose_multiple(rng, 4).cloned().collect())
    }

    pub fn from_string(s: String) -> Result<Self, String> {
        let mut vec = vec![];

        for c in s.trim().chars() {
            match c.to_digit(10) {
                Some(d) => vec.push(d as u8),
                None => return Err(format!("数字として解釈できない文字があります。c={}", c)),
            };
        }

        Ok(Code(vec))
    }
}

#[cfg(test)]
mod tests {
    use super::Code;
    use std::collections::HashSet;

    #[test]
    fn from_rand() {
        let mut rng = rand::thread_rng();
        let code = Code::from_rand(&mut rng);

        assert_eq!(code.0.len(), 4);

        let mut set = HashSet::new();

        for i in &code.0 {
            assert!(*i < 10);
            assert!(set.insert(*i));
        }
    }

    #[test]
    fn from_string() {
        assert_eq!(
            Code::from_string("0123".to_string()),
            Ok(Code(vec![0, 1, 2, 3]))
        );
        assert_eq!(
            Code::from_string("01a3".to_string()),
            Err("数字として解釈できない文字があります。c=a".to_string())
        );
    }
}
