use crossbeam_channel::{bounded, select, Receiver, Sender};

use std::thread;

use super::{CpuLikelihood, Likelihood};
use linalg::{RowMatrix, Vector};

type Row<const N: usize> = Vector<f64, N>;
type Substitution<const N: usize> = RowMatrix<f64, N, N>;

type Update<const N: usize> =
	(Vec<Substitution<N>>, Vec<usize>, Vec<(usize, usize)>);

pub struct ThreadedLikelihood<const N: usize> {
	propose: Sender<Update<N>>,
	likelihood: Receiver<f64>,
	accept: Sender<()>,
	reject: Sender<()>,
}

impl<const N: usize> Likelihood for ThreadedLikelihood<N> {
	type Row = Row<N>;
	type Substitution = Substitution<N>;

	fn propose(
		&mut self,
		substitutions: &[Self::Substitution],
		nodes: &[usize],
		children: &[(usize, usize)],
	) {
		self.propose
			.send((
				substitutions.to_vec(),
				nodes.to_vec(),
				children.to_vec(),
			))
			.unwrap();
	}

	fn likelihood(&self) -> f64 {
		self.likelihood.recv().unwrap()
	}

	fn accept(&mut self) {
		self.accept.send(()).unwrap()
	}

	fn reject(&mut self) {
		self.reject.send(()).unwrap()
	}
}

impl<const N: usize> ThreadedLikelihood<N> {
	#[allow(dead_code)]
	pub fn new(mut likelihood: CpuLikelihood<N>) -> Self {
		let (propose_sender, propose_reciever) =
			bounded::<Update<N>>(1);
		let (likelihood_sender, likelihood_reciever) = bounded(1);
		let (accept_sender, accept_reciever) = bounded(1);
		let (reject_sender, reject_reciever) = bounded(1);

		thread::spawn(move || loop {
			select! {
				recv(propose_reciever) -> update => {
					let Ok(update) = update else {
						break;
					};
					likelihood.propose(&update.0, &update.1, &update.2);
					let _ = likelihood_sender.send(likelihood.likelihood());
				}
				recv(accept_reciever) -> _ => {
					likelihood.accept();
				}
				recv(reject_reciever) -> _ => {
					likelihood.accept();
				}
			}
		});

		Self {
			propose: propose_sender,
			likelihood: likelihood_reciever,
			accept: accept_sender,
			reject: reject_sender,
		}
	}
}
