from scipy.stats import rv_continuous

from ._util import sample_range
from b3 import State, Proposal


class TreeSlide:
    def __init__(
        self,
        weight: float,
        distribution: rv_continuous,
    ):
        self.weight = weight
        self.distribution = distribution

    def propose(self, state: State) -> Proposal:
        tree = state.tree
        rng = state.rng

        node = tree.random_internal(rng)
        parent = tree.parent_of(node)
        if parent is None:
            return Proposal.Reject()

        left, right = tree.children_of(node)

        low = tree.weight_of(parent)
        high = min(tree.weight_of(left), tree.weight_of(right))

        new_weight = sample_range(low, high, self.distribution, rng.generator())

        tree.update_weight(node, new_weight)

        return Proposal.Hastings(0)
