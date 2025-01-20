#version 460

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

// TODO: restrict

layout(set = 0, binding = 0) buffer Length {
	uint num_rows;
};
layout(set = 0, binding = 1) buffer Probabilities {
	dvec4 probabilities[];
};
layout(set = 0, binding = 2) buffer Masks {
	uint masks[];
};

layout(set = 1, binding = 0) readonly buffer Nodes {
	uint nodes[];
};
layout(set = 1, binding = 1) readonly buffer Children {
	uint children[];
};
layout(set = 1, binding = 2) readonly buffer Substitutions {
	dmat4x4 substitutions[];
};

void main() {
	uint idx = gl_GlobalInvocationID.x;
	uint base = idx * num_rows;

	for (uint i = 0; i < nodes.length(); i++) {
		uint left_child = children[i * 2];
		uint right_child = children[i * 2 + 1];

		uint left_index = left_child * 2 +
			masks[base + left_child];
		uint right_index = right_child * 2 +
			masks[base + right_child];

		dvec4 left = substitutions[i * 2] *
			probabilities[base * 2 + left_index];
		dvec4 right = substitutions[i * 2 + 1] *
			probabilities[base * 2 + right_index];

		// flip the mask
		masks[base + nodes[i]] = 1 - masks[base + nodes[i]];
		uint parent_index = nodes[i] + masks[base + nodes[i]];
		// write the new value
		probabilities[base * 2 + parent_index] = left * right;
	}
}
