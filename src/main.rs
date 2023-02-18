extern crate rand;

mod check_result;
mod code;

use check_result::CheckResult;
use code::Code;
use std::io;
use std::io::Write;

fn main() {
    let len: usize = 10;
    let answer = match Code::from_rand(len) {
        Ok(code) => code,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut counter = 0;

    loop {
        print!("{}桁の数字を入力してください: ", len);
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

        counter += 1;
        println!("{}", result);

        if result.correct(len) {
            println!("Congratulations!");
            println!("Your score: {}", counter);
            break;
        }
    }
}
