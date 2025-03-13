mod distribution;
pub mod likelihood;
pub mod log;
pub mod mcmc;
pub mod model;
pub mod operator;
mod parameter;
pub mod prior;
mod rng;
mod state;
mod transitions;
mod tree;
pub mod util;

mod py_parameter;

pub use distribution::Distribution;
pub use parameter::Parameter;
pub use rng::Rng;
pub use state::State;
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

	m.add_class::<py_parameter::PyParameter>()?;
	m.add_class::<Rng>()?;

	Ok(())
}
