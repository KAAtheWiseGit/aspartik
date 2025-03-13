use pyo3::prelude::*;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};

use std::ops::{Deref, DerefMut};

#[derive(Debug, Serialize, Deserialize)]
#[pyclass]
pub struct Rng {
	inner: Pcg64,
}

#[pymethods]
impl Rng {
	#[new]
	pub fn new(seed: u64) -> Self {
		Rng {
			inner: Pcg64::seed_from_u64(seed),
		}
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
