mod distribution;
pub mod likelihood;
pub mod log;
pub mod mcmc;
pub mod model;
pub mod operator;
mod parameter;
pub mod prior;
mod state;
mod transitions;
mod tree;
pub mod util;

pub use distribution::Distribution;
pub use parameter::Parameter;
pub use state::State;
pub use transitions::Transitions;
pub use tree::Tree;

// test
use pyo3::prelude::*;

/// Documentation.
#[pyfunction]
fn hello_world(name: &str) -> PyResult<()> {
	println!("Hello, {}!", name);

	Ok(())
}

/// Short title.
///
/// Description.
#[pymodule]
fn b3(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(hello_world, m)?)?;

	Ok(())
}
