// XXX: should this include the tree?
#[derive(Debug, Clone)]
pub enum Parameter {
	Real(Param<f64>),
	Integer(Param<i64>),
	Boolean(Param<bool>),
}

#[derive(Debug, Clone)]
pub struct Param<T: Copy + PartialOrd> {
	pub values: Vec<T>,
	pub min: Option<T>,
	pub max: Option<T>,
}

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
