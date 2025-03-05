from b3 import State
from b3.tree import Edge
from b3.transitions import Matrix

# TDB

class SubstitutionModel:
    # operator updates: List[Edge] or List[Node]
    def update(self, state: State) -> List[Tuple[Matrix, Edge]]:
        ...

class ClockModel:
    def update(self, state: State) -> List[Tuple[float, Edge]]:
        ...
