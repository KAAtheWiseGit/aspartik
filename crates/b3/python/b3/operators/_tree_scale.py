from scipy.stats import rv_continuous
from math import log

from ._util import sample_range
from b3 import State, Proposal


class TreeScale:
    def __init__(self, factor: float, distribution: rv_continuous):
        if not 0 < factor < 1:
            raise ValueError(f"factor must be between 0 and 1, got {factor}")
        self.factor = factor
        self.distribution = distribution

    def propose(self, state: State) -> Proposal:
        tree = state.tree
        generator = state.rng.generator()

        low, high = self.factor, 1 / self.factor
        scale = sample_range(low, high, self.distribution, generator)

        for node in tree.node():
            new_weight = tree.weight_of(node) * scale
            tree.update_weight(node, new_weight)

        ratio = log(scale) * (tree.num_internals - 2)
        return Proposal.Hastings(ratio)
