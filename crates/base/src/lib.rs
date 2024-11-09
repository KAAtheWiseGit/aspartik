#![allow(dead_code)]

mod bases;
mod error;
pub mod seq;
pub mod substitution;
mod tree;

pub use bases::DnaNucleoBase;
pub use error::{Error, Result};
