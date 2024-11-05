// XXX: should this include the tree?
#[derive(Debug, Clone)]
pub enum Parameter {
	Real(Param<f64>),
	Integer(Param<i64>),
	Boolean(Param<bool>),
}

#[derive(Debug, Clone)]
pub struct Param<T: PartialOrd> {
	values: Vec<T>,
	min: Option<T>,
	max: Option<T>,
}
