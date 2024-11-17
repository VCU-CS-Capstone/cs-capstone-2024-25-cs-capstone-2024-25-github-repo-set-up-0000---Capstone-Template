use std::{error::Error, num::ParseIntError};

use thiserror::Error;

pub mod responses;
pub mod utils;
#[derive(Debug, Error)]
pub enum RedCapParseError {
    #[error("Invalid multi checkbox field: {input:?}, reason: {reason:?}")]
    InvalidMultiCheckboxField { input: String, reason: GenericError },
}
#[derive(Debug, Error)]
pub enum GenericError {
    #[error(transparent)]
    ParseNumber(#[from] ParseIntError),
    #[error("{0}")]
    Other(String),
}
