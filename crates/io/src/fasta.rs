use std::io::{BufRead, BufReader, Lines, Read};

use base::{
	seq::{Character, Seq},
	Result,
};

#[derive(Debug)]
pub struct Sequence<T: Character> {
	description: String,
	seq: Seq<T>,
}

pub struct FastaReader<T: Character, R: Read> {
	current: Option<Sequence<T>>,
	reader: Lines<BufReader<R>>,
}

impl<T: Character, R: Read> FastaReader<T, R> {
	pub fn new(reader: R) -> Self {
		FastaReader {
			current: None,
			reader: BufReader::new(reader).lines(),
		}
	}
}

impl<T: Character, R: Read> Iterator for FastaReader<T, R> {
	type Item = Result<Sequence<T>>;

	fn next(&mut self) -> Option<Result<Sequence<T>>> {
		loop {
			let Some(line) = self.reader.next() else {
				return self.current.take().map(Ok);
			};
			let line = match line {
				Ok(line) => line,
				Err(err) => return Some(Err(err.into())),
			};
			// skip comments and empty lines
			if line.starts_with(";") || line.trim().is_empty() {
				continue;
			}

			if line.starts_with(">") {
				let out = self.current.take();

				self.current = Some(Sequence {
					description: line.to_owned(),
					seq: Seq::new(),
				});

				if out.is_some() {
					return out.map(Ok);
				} else {
					continue;
				}
			}

			let seq: Seq<T> = match Seq::try_from(line.as_str()) {
				Ok(seq) => seq,
				Err(err) => return Some(Err(err)),
			};
			if let Some(sequence) = self.current.as_mut() {
				sequence.seq.append(seq)
			}
		}
	}
}
