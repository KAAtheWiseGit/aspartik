import math

from b3 import State, Tree, Proposal
from b3.tree import Node, Internal

class NarrowExchange:
    def __init__(self, weight):
        self.weight = weight

    def propose(self, state: State) -> Proposal:
        tree = state.tree

        if tree.num_internals() < 2:
            return Proposal.Reject

        grandparent = None
        while grandparent is None:
            internal = state.tree.sample_internal(state.rng)
            if is_grandparent(tree, internal):
                grandparent = internal

        left, right = tree.children_of(grandparent)
        if tree.weight_of(left) > tree.weight_of(right):
            parent, uncle = left, right
        elif tree.weight_of(right) > tree.weight_of(left):
            parent, uncle = right, left
        else:
            return Proposal.Reject

        parent, uncle = tree.as_internal(parent), tree.as_internal(uncle)
        # If the lower child isn't internal, abort.
        if parent is None:
            return Proposal.Reject

        num_grandparents_before = 0
        for node in tree.internals():
            if is_grandparent(node):
                num_grandparents_before += 1

        before = int(is_grandparent(tree, parent)) + int(is_grandparent(tree, uncle))

        if rng.random_bool(0.5):
            child = tree.children_of(parent)[0]
        else:
            child = tree.children_of(parent)[1]

        tree.swap_parents(uncle, child)

        aftter = int(is_grandparent(tree, parent)) + int(is_grandparent(tree, uncle))
        num_grandparents_after = num_grandparents_before - before + after
        ratio = math.log(num_grandparents_before / num_grandparents_after)

        return Proposal.Hastings(ratio)


def is_grandparent(tree: Tree, node: Internal) -> bool:
    left, right = tree.children_of(node)
    tree.is_internal(left) and tree.is_internal(right)
