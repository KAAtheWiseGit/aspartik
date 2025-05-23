use pyo3::prelude::*;

mod tracing;

#[pymodule(name = "_aspartik_rust_impl")]
fn aspartik(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
	tracing::init();

	m.add_submodule(&b3::pymodule(py)?)?;
	m.add_submodule(&data::pymodule(py)?)?;
	m.add_submodule(&io::pymodule(py)?)?;
	m.add_submodule(&rng::pymodule(py)?)?;
	m.add_submodule(&stats::pymodule(py)?)?;

	Ok(())
}
