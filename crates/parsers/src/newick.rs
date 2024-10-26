#![allow(unused)]

use std::collections::HashMap;

use base::newick::{Node, Tree};

pub fn parse_tree<S: AsRef<str>>(tree: S) -> Tree {
	let _s = tree.as_ref();

	todo!()
}

fn name(s: &str) -> (&str, &str) {
	if let Some(s) = s.strip_prefix('"') {
		// quoted
		let end = s.find('"').unwrap();

		(&s[..end], s[end..].trim_start())
	} else {
		// bare
		let end = s.find([',', ';', ')', ':']).unwrap_or(s.len());

		(s[..end].trim(), s[end..].trim())
	}
}

fn attributes(s: &str) -> (&str, &str) {
	// TODO: unclosed attributes
	// XXX: can attributes have nested brackets in them?
	let Some(end) = s.find(']') else {
		return ("", s);
	};

	(&s[1..end], s[end + 1..].trim_start())
}

fn distance(s: &str) -> (Option<f64>, &str) {
	let end = s
		.find(|ch: char| {
			!ch.is_ascii_digit()
				&& ch != '.' && ch != 'e' && ch != '-'
				&& ch != '+'
		})
		.unwrap_or(s.len());

	(str::parse(&s[..end]).ok(), s[end..].trim_start())
}

const EMPTY: String = String::new();

fn node<'a>(s: &'a str, tree: &mut Tree, parent: Option<usize>) -> &'a str {
	let mut s = s.trim();

	// Insert a placeholder node before we finish parsing all of the
	// descendants
	let this_idx = tree.push(Node::new(EMPTY, None, None, EMPTY));

	if s.starts_with('(') {
		s = &s[1..];
		while !s.starts_with(')') && !s.is_empty() {
			s = node(s, tree, Some(this_idx));
		}
		if s.starts_with(')') {
			s = &s[1..];
		}
	}

	let (name, s) = name(s);
	let s = s.trim_start_matches(':').trim();
	let (attributes, s) = attributes(s);
	let (distance, s) = distance(s);

	tree.set(
		Node::new(
			name.to_owned(),
			distance,
			parent,
			attributes.to_owned(),
		),
		this_idx,
	);

	s.trim_start_matches([',']).trim()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn p() {
		let s = "(:0.1,B:0.2,(C:0.3,D:0.4)E:0.5)F:0.0;";
		let mut tree = Tree::empty();
		node(s, &mut tree, None);
		println!("{:#?}", tree);
	}
}
