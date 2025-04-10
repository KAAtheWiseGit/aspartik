import os
# TODO: find a proper fix
os.environ["OPENBLAS_NUM_THREADS"] = "1"

from scipy.stats import uniform

import b3
from b3 import Tree, State, Rng, Parameter, Likelihood
from b3.priors import UniformPrior
from b3.operators import TreeScale, NarrowExchange, WideExchange, TreeSlide
from b3.substitutions import JC

rng = Rng(4)
tree = Tree(50, rng)
tree.verify()

params = [
    Parameter.Real(0.5),
    Parameter.Integer(0, 1, 2, 3),
    Parameter.Boolean(True, False),
]
state = State(tree, params, rng)
priors = [UniformPrior(params[0], 0, 1)]
operators = [
    NarrowExchange(weight=25.0),
    WideExchange(weight=25.0),
    TreeSlide(uniform, weight=48.0),
    TreeScale(0.1, uniform, weight=2.0),
]
likelihood = Likelihood(data="data/100.fasta", substitution=JC())

b3.run(10_000, state, priors, operators, likelihood)
