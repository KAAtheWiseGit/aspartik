from scipy.stats import gamma

import b3
from b3 import Tree, State, Rng, Parameter
from b3.priors import UniformPrior
from b3.operators import TreeScale

tree = Tree(["a", "b"], [1.0, 2.0, 3.0], [0, 1])
rng = Rng(4)
params = [
    Parameter.Real(0.5),
    Parameter.Integer(0, 1, 2, 3),
    Parameter.Boolean(True, False),
]
state = State(tree, params)
priors = [UniformPrior(params[0], 0, 1)]
operators = [TreeScale(0.1, gamma(2))]

b3.run(1, state, priors, operators)
