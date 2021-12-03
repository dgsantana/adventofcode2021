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
