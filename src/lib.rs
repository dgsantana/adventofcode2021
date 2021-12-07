use std::fs::read_to_string;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdventError {
    #[error("Invalid data")]
    InvalidData,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    IntParseError(#[from] std::num::ParseIntError),
}

pub type AdventResult<T> = Result<T, AdventError>;

pub fn read_input(day: u8, is_sample: bool) -> AdventResult<String> {
    let path = if is_sample {
        format!("inputs/day{}_sample.txt", day)
    } else {
        format!("inputs/day{}.txt", day)
    };
    Ok(read_to_string(path)?)
}

#[macro_export]
macro_rules! timed_run {
    ( $prefix:literal, $expression:expr ) => {{
        let start = std::time::Instant::now();
        let duration = start.elapsed();
        let result = $expression;
        println!("{} took ({:?})", $prefix, duration);
        result
    }};
}
