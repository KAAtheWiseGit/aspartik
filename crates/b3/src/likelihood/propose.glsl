#version 460

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Probabilities {
	dvec4 probabilities[];
	bool mask[];
	uint length;
} probabilities;

layout(set = 0, binding = 0) buffer Input {
	uint nodes[];
	uint children[];
	dmat4x4 substitutions[];

	uint length;
} input;

void main() {
	uint idx = gl_GlobalInvocationID.x;
	uint base = idx * (probabilities.length * 2);

	for (uint i = 0; i < input.length; i++) {
		uint left_index = children[i * 2] * 2 +
			int(probabilities.mask[children[i * 2]]);
		uint right_index = children[i * 2 + 1] * 2 +
			int(probabilities.mask[children[i * 2 + 1]]);

		dvec4 left = input.substitutions[i * 2] *
			probabilities.probabilities[left_index];
		dvec4 right = input.substitutions[i * 2 + 1] *
			probabilities.probabilities[right_index];

		// flip the mask
		probabilities.mask[nodes[i]] = !probabilities.mask[nodes[i]];
		uint parent_index = nodes[i] + int(probabilities.mask[nodes[i]]);
		// write the new value
		probabilities[parent_index] = left * right;
	}
}
