use crossbeam_channel::{bounded, select, Receiver, Sender};

use std::{marker::PhantomData, thread};

use super::{CpuLikelihood, Likelihood};
use crate::tree::Update;
use base::substitution::Model;

pub struct ThreadedLikelihood<M: Model> {
	propose: Sender<Update>,
	likelihood: Receiver<f64>,
	accept: Sender<()>,
	reject: Sender<()>,

	marker: PhantomData<M>,
}

impl<M: Model + 'static> Likelihood for ThreadedLikelihood<M> {
	type Model = M;

	fn new(
		sites: Vec<Vec<<Self::Model as Model>::Item>>,
		model: Self::Model,
	) -> Self {
		let mut likelihood = CpuLikelihood::new(sites, model);

		let (propose_sender, propose_reciever) = bounded::<Update>(1);
		let (likelihood_sender, likelihood_reciever) = bounded(1);
		let (accept_sender, accept_reciever) = bounded(1);
		let (reject_sender, reject_reciever) = bounded(1);

		thread::spawn(move || loop {
			select! {
				recv(propose_reciever) -> update => {
					let Ok(update) = update else {
						break;
					};
					likelihood.propose(update);
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

			marker: PhantomData,
		}
	}

	fn propose(&mut self, update: Update) {
		self.propose.send(update).unwrap();
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
