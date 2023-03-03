extern crate rand;

mod factories;
mod features;
mod libs;
mod models;

use std::io::{self, Write};

use clap::Parser;
use factories::CodeFactory;
use models::Possibility;

use crate::features::guess::guess;

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
    possibility: bool,
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

    let mut possibility: Possibility = if args.possibility {
        factory.generate_all(args.length).into()
    } else {
        Possibility::new()
    };

    let answer = factory.generate(args.length);
    let mut counter = 0;

    loop {
        print!("{}桁の数字を入力してください: ", args.length);
        io::stdout().flush().unwrap();

        let (log, is_correct) = match guess(&factory, &answer) {
            Ok(r) => r,
            Err(e) => {
                println!("{}\nもう一度入力してください。", e);
                continue;
            }
        };

        counter += 1;
        println!("{}", log);

        if args.possibility {
            possibility.update(&log);
            println!("{}", possibility);
        }

        if is_correct {
            println!("Congratulations!");
            println!("Your score: {}", counter);
            break;
        }
    }
}
