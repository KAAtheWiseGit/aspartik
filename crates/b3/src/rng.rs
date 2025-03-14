use anyhow::Result;
use pyo3::prelude::*;
use pyo3::{ffi::c_str, types::PyDict};
use rand::{Rng as _, SeedableRng};
use rand_pcg::Pcg64;

use std::{
	ffi::CStr,
	sync::{Arc, Mutex, MutexGuard},
};

pub type Rng = Pcg64;

#[derive(Debug, Clone)]
#[pyclass(frozen)]
pub struct PyRng {
	inner: Arc<Mutex<Rng>>,
}

impl PyRng {
	pub fn inner(&self) -> MutexGuard<Pcg64> {
		self.inner.lock().unwrap()
	}
}

const NEW_GENERATOR: &CStr = c_str!(r#"
import numpy.random

rng = numpy.random.default_rng(seed)
"#);

#[pymethods]
impl PyRng {
	#[new]
	pub fn new(seed: u64) -> Self {
		let inner = Pcg64::seed_from_u64(seed);
		PyRng {
			inner: Arc::new(Mutex::new(inner)),
		}
	}

	/// Creates a new NumPy `Generator`.
	///
	/// This method should always be used to create random samplers for
	/// operators, since `Rng` is seeded and its internal state is tracked
	/// in the simulation state.
	fn generator(&self, py: Python) -> Result<PyObject> {
		let seed: u64 = self.inner().random();

		let locals = PyDict::new(py);
		locals.set_item("seed", seed)?;
		py.run(NEW_GENERATOR, None, Some(&locals))?;
		let rng = locals.get_item("rng")?.unwrap();

		Ok(rng.into())
	}
}
