extern crate rand;

mod check_result;
mod code;

use check_result::CheckResult;
use clap::Parser;
use code::CodeFactory;
use std::io;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of length of code
    #[arg(short, long, default_value_t = 4)]
    length: usize,
    #[arg(short, long, default_value_t = 10)]
    radix: u8,
}

fn main() {
    let args = Args::parse();
    let factory = match CodeFactory::try_from(args.radix) {
        Ok(factory) => factory,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let answer = match factory.generate(args.length) {
        Ok(code) => code,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut counter = 0;

    loop {
        print!("{}桁の数字を入力してください: ", args.length);
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("入力エラー。read_line()で失敗しました。");

        let guess = match factory.generate_from_string(guess) {
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

        if result.correct(args.length) {
            println!("Congratulations!");
            println!("Your score: {}", counter);
            break;
        }
    }
}
