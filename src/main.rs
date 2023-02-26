extern crate rand;

mod factories;
mod features;
mod libs;
mod models;

use clap::Parser;
use factories::CodeFactory;
use std::io;
use std::io::Write;

use crate::features::calculator::calculator;
use crate::models::Log;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of length of code
    #[arg(short, long, default_value_t = 4)]
    length: usize,

    /// Number of radix [10, 16]
    #[arg(short, long, default_value_t = 10)]
    radix: u8,

    /// Number of radix [10, 16]
    #[arg(short, long, default_value_t = false)]
    calc: bool,
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

    let mut patterns = if args.calc {
        factory.generate_all(args.length)
    } else {
        vec![]
    };

    let answer = factory.generate(args.length);
    let mut counter = 0;

    loop {
        print!("{}桁の数字を入力してください: ", args.length);
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("入力エラー。read_line()で失敗しました。");

        let guess = match factory.generate_from_str(&guess) {
            Ok(guess) => guess,
            Err(e) => {
                println!("{}\nもう一度入力してください。", e);
                continue;
            }
        };

        let (result, is_correct) = match answer.diff(&guess) {
            Ok(d) => d,
            Err(e) => {
                println!("{}\nもう一度入力してください。", e);
                continue;
            }
        };

        let log = Log { guess, result };

        counter += 1;
        println!("{}", log.result);

        if args.calc {
            patterns = calculator(patterns, &log);

            println!("counts: {}", patterns.len());

            print!("possibilities: ");
            for code in patterns.get(0..5).unwrap_or(&patterns) {
                print!("{}, ", code);
            }
            if patterns.len() > 5 {
                print!(" etc...");
            }
            println!();
        }

        if is_correct {
            println!("Congratulations!");
            println!("Your score: {}", counter);
            break;
        }
    }
}
