#version 460

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) readonly restrict buffer Length {
	uint num_rows;
};
layout(set = 0, binding = 1) restrict buffer Probabilities {
	dvec4 probabilities[];
};
layout(set = 0, binding = 2) restrict buffer Masks {
	uint masks[];
};

layout(set = 1, binding = 0) restrict readonly buffer Root {
	uint root;
};
layout(set = 1, binding = 1) restrict buffer Sums {
	double sums[];
};

void main() {
	uint idx = gl_GlobalInvocationID.x;
	uint offset = idx * num_rows;

	if (offset >= masks.length()) {
		return;
	}

	// the masks start at offset
	// the probabilities start at offset * 2

	uint mask = masks[offset + root];
	dvec4 probability = probabilities[offset * 2 + root * 2 + mask];
	double sum = probability.x + probability.y + probability.z + probability.w;
	sums[idx] = sum;
}
