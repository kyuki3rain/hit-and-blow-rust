use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

pub struct Code {
    pub code: Vec<u8>,
}

impl Code {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let choices = (0..10).collect::<Vec<u8>>();
        return Self {
            code: choices.choose_multiple(rng, 4).cloned().collect(),
        };
    }

    pub fn from_string(s: String) -> Result<Self, String> {
        let mut vec = vec![];

        for c in s.trim().chars() {
            match c.to_digit(10) {
                Some(d) => vec.push(d as u8),
                None => return Err(format!("数字として解釈できない文字があります。c={}", c)),
            };
        }

        Ok(Self { code: vec })
    }
}
