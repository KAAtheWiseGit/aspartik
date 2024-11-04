use thiserror::Error;

use std::io;

#[derive(Error, Debug)]
pub enum Error {
	#[error("'{0}' is not a valid IUPAC character")]
	InvalidNucleoBaseChar(char),
	#[error("invalid base value: {0}")]
	InvalidNucleoBaseByte(u8),

	#[error("IO error: {0}")]
	IO(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
