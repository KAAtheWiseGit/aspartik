pub mod clock;
pub mod data;
// pub mod likelihood;
pub mod log;
// pub mod mcmc;
pub mod mcmc2;
pub mod operator;
pub mod parameter;
pub mod prior;
pub mod rng;
mod state;
pub mod substitution;
mod transitions;
mod tree;
pub mod util;

pub use prior::PyPrior;
pub use transitions::Transitions;
pub use tree::Tree;

use pyo3::prelude::*;

/// Short title.
///
/// Description.
#[pymodule]
fn b3(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
	let tree = tree::submodule(py)?;
	m.add_submodule(&tree)?;
	py.import("sys")?
		.getattr("modules")?
		.set_item("b3.tree", tree)?;

	let log = log::submodule(py)?;
	m.add_submodule(&log)?;
	py.import("sys")?
		.getattr("modules")?
		.set_item("b3.log", log)?;

	m.add_class::<parameter::PyParameter>()?;
	m.add_class::<rng::PyRng>()?;
	m.add_class::<state::PyState>()?;
	m.add_class::<tree::PyTree>()?;
	m.add_class::<operator::Proposal>()?;
	// XXX: submodule?
	m.add_class::<data::PyDna>()?;

	m.add_function(wrap_pyfunction!(mcmc2::run, m)?)?;

	Ok(())
}
