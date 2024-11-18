use std::fmt::{Result, Write};

use super::{Node, NodeIndex, Tree};

impl Node {
	pub fn serialize_to<W: Write>(&self, writer: &mut W) -> Result {
		if self.name.contains(['"', ',', ';', '[', ' ', '\t']) {
			writer.write_char('"')?;
			writer.write_str(&self.name.replace("\"", "\\\""))?;
			writer.write_char('"')?;
		} else {
			writer.write_str(&self.name)?;
		}

		Ok(())
	}
}

impl Tree {
	pub fn serialize_to<W: Write>(&self, writer: &mut W) -> Result {
		let Some(root) = self.root else {
			todo!("Figure out what to do for unrooted trees")
		};

		serialize(self, root, writer)
	}

	pub fn serialize(&self) -> String {
		let mut out = String::new();
		self.serialize_to(&mut out)
			.expect("Writing to `String` should be infallible");
		out
	}
}

/// Recursive.
fn serialize<W: Write>(tree: &Tree, node: NodeIndex, writer: &mut W) -> Result {
	let children = tree.children_of(node);

	if !children.is_empty() {
		writer.write_char('(')?;

		for child in children {
			serialize(tree, child, writer)?;
		}

		writer.write_char(')')?;
	}

	tree.get_node(node).serialize_to(writer)
}
