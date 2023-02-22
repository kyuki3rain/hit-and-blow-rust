pub mod code_factory;
pub use code_factory::CodeFactory;

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
}
