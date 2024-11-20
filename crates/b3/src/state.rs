use serde_json::{json, Value as Json};

use std::{
	collections::HashMap,
	sync::mpsc::{sync_channel, Receiver, SyncSender},
	thread,
};

use crate::{
	likelihood::Likelihood,
	operator::Proposal,
	parameter::{BooleanParam, IntegerParam, Parameter, RealParam},
	tree::{Tree, Update},
};
use base::{seq::DnaSeq, substitution::dna::Dna4Substitution};

pub struct State {
	/// Map of parameters by name.
	params: HashMap<String, Parameter>,
	/// Proposal parameters
	proposal_params: HashMap<String, Parameter>,
	/// The phylogenetic tree, which also contains the genetic data.
	tree: Tree,

	senders: Vec<SyncSender<Update>>,
	recievers: Vec<Receiver<f64>>,
	verdicts: Vec<SyncSender<Verdict>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
	Accept,
	Reject,
}

impl State {
	pub fn new(tree: Tree, sequences: &[DnaSeq]) -> State {
		let mut c1 = vec![];
		let mut c2 = vec![];
		let mut c3 = vec![];
		let mut c4 = vec![];

		for i in 0..sequences[0].len() {
			let mut column = Vec::new();
			for sequence in sequences {
				column.push(sequence[i]);
			}

			if i % 4 == 0 {
				c1.push(column);
			} else if i % 4 == 1 {
				c2.push(column);
			} else if i % 4 == 2 {
				c3.push(column);
			} else {
				c4.push(column);
			}
		}

		let mut likelihoods = vec![
			Likelihood::new(c1, Dna4Substitution::jukes_cantor()),
			Likelihood::new(c2, Dna4Substitution::jukes_cantor()),
			Likelihood::new(c3, Dna4Substitution::jukes_cantor()),
			Likelihood::new(c4, Dna4Substitution::jukes_cantor()),
		];

		let update = tree.update_all_likelihoods();
		for likelihood in &mut likelihoods {
			likelihood.update(&update);
		}

		let mut senders = vec![];
		let mut recievers = vec![];
		let mut verdicts = vec![];
		while let Some(mut likelihood) = likelihoods.pop() {
			let (upd_send, upd_recv) = sync_channel::<Update>(1);
			let (like_send, like_recv) = sync_channel::<f64>(1);
			let (ver_send, ver_recv) = sync_channel::<Verdict>(1);

			thread::spawn(move || {
				likelihood.spin(upd_recv, like_send, ver_recv);
			});

			senders.push(upd_send);
			recievers.push(like_recv);
			verdicts.push(ver_send);
		}

		State {
			params: HashMap::new(),
			proposal_params: HashMap::new(),
			tree,
			senders,
			recievers,
			verdicts,
		}
	}

	pub fn likelihood(&self) -> f64 {
		let mut out = 0.0;

		for recv in &self.recievers {
			out += recv.recv().unwrap();
		}

		out
	}

	/// # Panics
	///
	/// Panics if `name` is not a valid parameter name.
	pub fn get_parameter<S: AsRef<str>>(&self, name: S) -> &Parameter {
		let name = name.as_ref();

		if let Some(param) = self.proposal_params.get(name) {
			return param;
		}

		&self.params[name]
	}

	pub fn get_real_parameter<S: AsRef<str>>(
		&self,
		name: S,
	) -> Option<&RealParam> {
		match self.get_parameter(name) {
			Parameter::Real(p) => Some(p),
			_ => None,
		}
	}

	pub fn get_integer_parameter<S: AsRef<str>>(
		&self,
		name: S,
	) -> Option<&IntegerParam> {
		match self.get_parameter(name) {
			Parameter::Integer(p) => Some(p),
			_ => None,
		}
	}

	pub fn get_boolean_parameter<S: AsRef<str>>(
		&self,
		name: S,
	) -> Option<&BooleanParam> {
		match self.get_parameter(name) {
			Parameter::Boolean(p) => Some(p),
			_ => None,
		}
	}

	pub fn has_parameter<S: AsRef<str>>(&self, name: S) -> bool {
		// Proposal can't have parameters not already in state.
		self.params.contains_key(name.as_ref())
	}

	pub fn get_tree(&self) -> &Tree {
		&self.tree
	}

	pub fn propose(&mut self, mut proposal: Proposal) {
		self.proposal_params = std::mem::take(&mut proposal.params);

		let update = self.tree.propose(proposal);

		for sender in &self.senders {
			sender.send(update.clone()).unwrap();
		}
	}

	/// Accept the current proposal
	pub fn accept(&mut self) {
		for (name, param) in std::mem::take(&mut self.proposal_params) {
			self.params.insert(name, param);
		}

		self.tree.accept();

		for sender in &self.verdicts {
			sender.send(Verdict::Accept).unwrap();
		}
	}

	pub fn reject(&mut self) {
		self.proposal_params.clear();

		self.tree.reject();

		for sender in &self.verdicts {
			sender.send(Verdict::Reject).unwrap();
		}
	}

	pub fn serialize(&self) -> Json {
		json!({
			"tree": self.tree.serialize(),
			"parameters": self.params,
		})
	}
}
