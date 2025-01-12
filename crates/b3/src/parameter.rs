use serde::{Deserialize, Serialize};

use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param<T: Copy + PartialOrd> {
	pub values: Vec<T>,
	pub min: Option<T>,
	pub max: Option<T>,
}

pub type RealParam = Param<f64>;
pub type IntegerParam = Param<i64>;
pub type BooleanParam = Param<bool>;

impl<T: Copy + PartialOrd> Param<T> {
	/// Returns `false` if any value inside the parameter is out of bounds.
	pub fn is_valid(&self) -> bool {
		self.values.iter().all(|val| {
			let lower =
				self.min.map(|min| min <= *val).unwrap_or(true);
			let upper =
				self.max.map(|max| *val <= max).unwrap_or(true);

			// Both the upper and the lower bounds are either
			// satisfied or not present
			lower && upper
		})
	}

	/// Get the first value of the parameter.
	pub fn first(&self) -> T {
		*self.values
			.first()
			.expect("Parameters must have at least one dimension")
	}

	pub fn len(&self) -> usize {
		self.values.len()
	}
}

impl<T: Copy + PartialOrd> Index<usize> for Param<T> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		&self.values[index]
	}
}

impl<T: Copy + PartialOrd> IndexMut<usize> for Param<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.values[index]
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Parameter {
	Real(RealParam),
	Integer(IntegerParam),
	Boolean(BooleanParam),
}

impl Parameter {
	/// Checks that the parameter is valid: all of its values must lie
	/// within bounds.
	pub fn is_valid(&self) -> bool {
		match self {
			Parameter::Real(p) => p.is_valid(),
			Parameter::Integer(p) => p.is_valid(),
			Parameter::Boolean(p) => p.is_valid(),
		}
	}

	pub fn len(&self) -> usize {
		match self {
			Parameter::Real(p) => p.len(),
			Parameter::Integer(p) => p.len(),
			Parameter::Boolean(p) => p.len(),
		}
	}

	pub fn as_real(&self) -> Option<&RealParam> {
		match self {
			Parameter::Real(p) => Some(p),
			_ => None,
		}
	}

	pub fn as_integer(&self) -> Option<&IntegerParam> {
		match self {
			Parameter::Integer(p) => Some(p),
			_ => None,
		}
	}

	pub fn as_boolean(&self) -> Option<&BooleanParam> {
		match self {
			Parameter::Boolean(p) => Some(p),
			_ => None,
		}
	}

	pub fn as_mut_real(&mut self) -> Option<&mut RealParam> {
		match self {
			Parameter::Real(p) => Some(p),
			_ => None,
		}
	}

	pub fn as_mut_integer(&mut self) -> Option<&mut IntegerParam> {
		match self {
			Parameter::Integer(p) => Some(p),
			_ => None,
		}
	}

	pub fn as_mut_boolean(&mut self) -> Option<&mut BooleanParam> {
		match self {
			Parameter::Boolean(p) => Some(p),
			_ => None,
		}
	}
}
