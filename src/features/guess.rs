use std::io;

use crate::factories::CodeFactory;
use crate::models::{Code, Log};

pub fn guess(factory: &CodeFactory, answer: &Code) -> Result<(Log, bool), String> {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("入力エラー。read_line()で失敗しました。");

    let guess = factory.generate_from_str(&guess)?;
    let (result, is_correct) = answer.diff(&guess)?;

    Ok((Log { guess, result }, is_correct))
}
