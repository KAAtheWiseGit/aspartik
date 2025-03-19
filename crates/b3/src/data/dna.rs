use anyhow::Result;
use pyo3::prelude::*;
use pyo3::types::{PySlice, PySliceIndices, PyTuple};

use std::{
	fs::File,
	path::{Path, PathBuf},
};

use base::{seq::DnaSeq, DnaNucleoBase};
use io::fasta::FastaReader;

#[pyclass]
#[pyo3(name = "DNA")]
pub struct PyDna {
	seqs: Vec<DnaSeq>,
	names: Vec<String>,
}

impl PyDna {
	fn from_fasta_dna(path: &Path) -> Result<Self> {
		let file = File::open(path)?;
		let fasta = FastaReader::<DnaNucleoBase, _>::new(file);

		let mut seqs = Vec::<DnaSeq>::new();
		let mut names = Vec::new();
		for sample in fasta {
			let sample = sample?;
			names.push(sample.description().to_owned());
			seqs.push(sample.into());
		}

		Ok(Self { seqs, names })
	}

	fn ilen(&self) -> isize {
		self.seqs.len() as isize
	}

	fn slice(&self, indices: Vec<PySliceIndices>) -> Self {
		let length: usize = indices.iter().map(|i| i.slicelength).sum();
		let mut seqs = Vec::with_capacity(length);
		let mut names = Vec::with_capacity(length);

		for indice in indices {
			// TODO: proper conversion without panic
			let mut i = indice.start as usize;
			while i < indice.stop as usize {
				seqs.push(self.seqs[i].clone());
				names.push(self.names[i].clone());
				i += indice.step as usize;
			}
		}

		Self { seqs, names }
	}
}

#[pymethods]
impl PyDna {
	#[new]
	fn new(path: PathBuf) -> Result<Self> {
		// XXX: different formats, format detection
		Self::from_fasta_dna(&path)
	}

	fn __getitem__(&self, key: Bound<PyAny>) -> Result<PyDna> {
		if let Ok(slice) = key.downcast::<PySlice>() {
			let indice = slice.indices(self.ilen())?;
			Ok(self.slice(vec![indice]))
		} else if let Ok(tuple) = key.downcast::<PyTuple>() {
			let mut indices = Vec::new();
			for item in tuple.into_iter() {
				// TODO: `Send` error
				let slice = item.downcast::<PySlice>().unwrap();
				let indice = slice.indices(self.ilen())?;
				indices.push(indice);
			}
			Ok(self.slice(indices))
		} else {
			todo!("type error")
		}
	}

	fn __str__(&self) -> String {
		self.names.join("\n")
	}
}
