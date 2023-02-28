use crate::models::{Log, Possibility};

pub fn calculator(prev: Possibility, log: &Log) -> Possibility {
    let mut calc_result = Possibility::new();

    for code in prev {
        if let Ok((result, _)) = code.diff(&log.guess) {
            if result == log.result {
                calc_result.push(code);
            }
        }
    }

    calc_result
}
