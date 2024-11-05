pub enum Proposal {
	Accept,
	Reject,
	// TODO: decide if it should be log or not
	Hastings(f64),
}

pub trait Operator {
	// TODO: rng
	fn propose() -> Proposal;
}
