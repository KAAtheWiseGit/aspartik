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

	#[test]
	fn tree() {
		let s = "((11:78.51463972926828,(((9:7.687822300343535,(8:1.2997671677365752,7:1.2997671677365752):6.38805513260696):2.5517113635399182,10:10.239533663883453):47.83909385190117,(((4:10.303956314457084,(3:7.014862584373447,2:7.014862584373447):3.2890937300836365):10.890052001978896,5:21.19400831643598):8.165283167190701,6:29.35929148362668):28.719336032157944):20.436012213483657):16.847636009595632,(1:74.35882993398783,12:74.35882993398783):21.00344580487608):0.0;";
		let mut tree = Tree::empty();
		node(s, &mut tree, None);
		println!("{:#?}", tree);
	}
}
