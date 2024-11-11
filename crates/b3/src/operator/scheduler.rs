use super::Operator;

pub struct TurnScheduler {
	operators: Vec<Box<dyn Operator>>,
	current: usize,
}

impl TurnScheduler {
	pub fn new<I>(operators: I) -> Self
	where
		I: IntoIterator<Item = Box<dyn Operator>>,
	{
		Self {
			operators: operators.into_iter().collect(),
			current: 0,
		}
	}

	pub fn get_operator(&mut self) -> &Box<dyn Operator> {
		let out = &self.operators[self.current];

		self.current = (self.current + 1) % self.operators.len();

		out
	}
}
