extern crate rand;

use std::fmt;
use std::io;
use std::io::Write;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

pub struct Code {
    pub code: Vec<u8>,
}

pub struct CheckResult {
    hit: u8,
    blow: u8,
}

impl CheckResult {
    pub fn check(answer: &Code, guess: &Code) -> Result<Self, String> {
        let mut hit = 0;
        let mut blow = 0;

        if answer.code.len() != guess.code.len() {
            return Err(format!(
                "長さが間違っています。ans={}, guess={}",
                answer.code.len(),
                guess.code.len()
            ));
        }

        for (idx, &val) in guess.code.iter().enumerate() {
            if let Some(ans_idx) = answer.code.iter().position(|&ans| ans == val) {
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

fn main() {
    let mut rng = rand::thread_rng();
    let answer = Code::new(&mut rng);

    loop {
        print!("4桁の数字を入力してください: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("入力エラー。read_line()で失敗しました。");

        let guess = match Code::from_string(guess) {
            Ok(guess) => guess,
            Err(e) => {
                println!("{}\nもう一度入力してください。", e);
                continue;
            }
        };

        let result = match CheckResult::check(&answer, &guess) {
            Ok(result) => result,
            Err(e) => {
                println!("{}\nもう一度入力してください。", e);
                continue;
            }
        };

        println!("{}", result);

        if result.correct() {
            println!("Congratulations!!");
            break;
        }
    }
}
