#![allow(dead_code)]

mod bases;
mod error;
pub mod seq;
pub mod substitution;

pub use bases::DnaNucleoBase;
pub use error::{Error, Result};
