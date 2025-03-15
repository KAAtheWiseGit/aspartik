from b3 import Tree, State, Rng, Parameter

tree = Tree(["a", "b"], [1.0, 2.0, 3.0], [0, 1])
rng = Rng(4)
params = [
    Parameter.Real(0.5),
    Parameter.Integer(0, 1, 2, 3),
    Parameter.Boolean(True, False),
]
state = State(tree, params)
