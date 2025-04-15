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

#[derive(Debug)]
#[pyclass(name = "Rng", frozen)]
pub struct PyRng {
	inner: Arc<Mutex<Rng>>,
	scipy: PyObject,
}

impl PyRng {
	pub fn clone_with(&self, py: Python) -> Self {
		Self {
			inner: self.inner.clone(),
			scipy: self.scipy.clone_ref(py),
		}
	}
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

fn make_scipy_genrator(rng: &mut Rng, py: Python) -> Result<PyObject> {
	let seed: u64 = rng.random();

	let locals = PyDict::new(py);
	locals.set_item("seed", seed)?;
	py.run(NEW_GENERATOR, None, Some(&locals))?;
	let rng = locals.get_item("rng")?.unwrap();

	Ok(rng.into())
}

#[pymethods]
impl PyRng {
	#[new]
	fn new(py: Python, seed: u64) -> Result<Self> {
		let mut inner = Pcg64::seed_from_u64(seed);
		let scipy = make_scipy_genrator(&mut inner, py)?;
		Ok(PyRng {
			inner: Arc::new(Mutex::new(inner)),
			scipy,
		})
	}

	fn random_bool(&self, ratio: f64) -> bool {
		self.inner().random_bool(ratio)
	}

	fn generator(&self, py: Python) -> PyObject {
		self.scipy.clone_ref(py)
	}
}
