#[derive(Debug, Clone)]
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
		self.values
			.iter()
			.map(|val| {
				let lower =
					self.min.map(|min| min <= *val)
						.unwrap_or(true);
				let upper =
					self.max.map(|max| *val <= max)
						.unwrap_or(true);

				upper && lower
			})
			// all items are true
			.all(|x| x)
	}
}

#[derive(Debug, Clone)]
pub enum Parameter {
	Real(RealParam),
	Integer(IntegerParam),
	Boolean(BooleanParam),
}

impl Parameter {
	pub fn is_valid(&self) -> bool {
		match self {
			Parameter::Real(p) => p.is_valid(),
			Parameter::Integer(p) => p.is_valid(),
			Parameter::Boolean(p) => p.is_valid(),
		}
	}
}
