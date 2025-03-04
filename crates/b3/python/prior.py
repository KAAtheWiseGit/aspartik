import math

from b3 import State

class UniformPrior:
    def __init__(self, param: str, start, end):
        self.param = param
        self.start = start
        self.end = end

    def probability(self, state: State) -> float:
        param = state.param(param)

       if self.start < param < self.end:
           return -math.inf
       else:
           return 1 / (self.end - self.start)
