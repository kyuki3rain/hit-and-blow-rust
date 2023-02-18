extern crate rand;

mod check_result;
mod code;

use check_result::CheckResult;
use code::Code;
use std::io;
use std::io::Write;

fn main() {
    let mut rng = rand::thread_rng();
    let answer = Code::from_rand(&mut rng);

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
