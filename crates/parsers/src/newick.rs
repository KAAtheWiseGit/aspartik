#![allow(unused)]

use std::collections::HashMap;

use base::newick::{Attributes, Node, Tree};

pub fn parse_tree<S: AsRef<str>>(tree: S) -> Tree {
	let _s = tree.as_ref();

	todo!()
}

fn parse_node<'a>(
	s: &'a str,
	tree: &mut Tree,
	parent: Option<usize>,
) -> &'a str {
	let mut s = s.trim();

	if s.starts_with('(') {
		// TODO
	}

	let name: Option<String> = if s.starts_with('"') {
		// Quoted name

		// TODO: error on malformed unclosed quotes
		let end = s[1..].find('"').unwrap();
		let name_s = &s[1..end];
		s = &s[end..];
		Some(name_s.trim_matches('"').to_owned())
	} else if s.starts_with(':') {
		// Anonymous node
		s = &s[1..];
		None
	} else {
		// Bare name
		if let Some((bare_name, rest)) = s.split_once(':') {
			// With distance
			s = rest;
			Some(bare_name.trim().to_owned())
		} else {
			// No distance
			s = "";
			None
		}
	};

	s = s.trim();
	let attributes = if s.starts_with("[") {
		// TODO: error on malformed unclosed attributes
		let (_attr_s, rest) = s.split_once(']').unwrap();
		s = rest;

		// TODO: parse attributes
		HashMap::new()
	} else {
		HashMap::new()
	};

	s = s.trim();
	let end = s
		.char_indices()
		.take_while(|(i, ch)| ch.is_ascii_alphanumeric() || *ch == '.')
		.last()
		.map(|(i, _)| i)
		.unwrap_or(0);
	let distance = str::parse(&s[..end]).ok();
	s = &s[end..];

	tree.push(Node::new(name, distance, parent, attributes));

	s
}

fn find_matching_paren(s: &str) -> usize {
	let mut num_parens = 0;
	let mut num_brackets = 0;

	let mut out = 0;

	for ch in s.chars() {
		if num_brackets > 0 {
			if ch == '[' {
				num_brackets += 1;
			} else if ch == ']' {
				// TODO: this might underflow if the comments
				// aren't well-formed
				num_brackets -= 1;
			}
		} else {
			if ch == '[' {}
		}
	}

	out
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn p() {
		let s = "(A:0.1,B:0.2,(C:0.3,D:0.4)E:0.5)F";
	}

	#[test]
	fn node() {
		let s = "A : [&attrs...] 0.5";
		let mut tree = Tree::empty();
		parse_node(s, &mut tree, None);

		println!("{:#?}", tree);
	}
}
