from b3 import State

import math


class UniformPrior:
    def __init__(self, param, start, end):
        self.param = param
        self.start = start
        self.end = end

    def probability(self, state: State) -> float:
        if self.start < param < self.end:
            return 1 / (self.end - self.start)
        else:
            return -math.inf
