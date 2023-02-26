use crate::models::{Code, Log};

pub fn calculator(prev: Vec<Code>, log: &Log) -> Vec<Code> {
    let mut calc_result: Vec<Code> = vec![];

    for code in prev {
        if let Ok((result, _)) = code.diff(&log.guess) {
            if result == log.result {
                calc_result.push(code);
            }
        }
    }

    calc_result
}
