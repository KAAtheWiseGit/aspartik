from scipy.stats import rv_continuous

from ._util import sample_range
from b3 import State, Proposal


class TreeScale:
    def __init__(self, factor: float, distribution: rv_continuous, weight: float):
        if not 0 < factor < 1:
            raise ValueError(f"factor must be between 0 and 1, got {factor}")
        self.weight = weight

    def propose(self, state: State) -> Proposal:
        tree = state.tree
        generator = state.rng.generator()

        scale = sample_range(factor, 1/factor, self.distribution, generator)

        for node in tree.node():
            new_weight = tree.weight_of(node) * scale
            tree.update_weight(node, new_weight)

        ratio = scale.ln() * (tree.num_internals - 2)
        return Proposal.Hastings(ratio)
