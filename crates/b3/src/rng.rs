use anyhow::Result;
use pyo3::prelude::*;
use pyo3::{ffi::c_str, types::PyDict};
use rand::{Rng as _, SeedableRng};
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};

use std::{
	ffi::CStr,
	ops::{Deref, DerefMut},
};

#[derive(Debug, Serialize, Deserialize)]
#[pyclass]
pub struct Rng {
	inner: Pcg64,
}

const NEW_GENERATOR: &CStr = c_str!(r#"
import numpy.random

rng = numpy.random.default_rng(seed)
"#);

#[pymethods]
impl Rng {
	#[new]
	pub fn new(seed: u64) -> Self {
		Rng {
			inner: Pcg64::seed_from_u64(seed),
		}
	}

	/// Creates a new NumPy `Generator`.
	///
	/// This method should always be used to create random samplers for
	/// operators, since `Rng` is seeded and its internal state is tracked
	/// in the simulation state.
	fn generator(&mut self, py: Python) -> Result<PyObject> {
		let seed: u64 = self.random();

		let locals = PyDict::new(py);
		locals.set_item("seed", seed)?;
		py.run(NEW_GENERATOR, None, Some(&locals))?;
		let rng = locals.get_item("rng")?.unwrap();

		Ok(rng.into())
	}
}

impl Deref for Rng {
	type Target = Pcg64;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for Rng {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}
