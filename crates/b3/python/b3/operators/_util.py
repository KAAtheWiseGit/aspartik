from math import inf, exp
from scipy.stats import rv_continuous
from numpy.random import Generator


# x must be in [0, inf)
def interval_to_range(ratio, low, high):
    return low + (high - low) / (ratio + 1)


def rescale_range(x, pre_low, pre_high, low, high):
    ratio = (x - pre_low) / (pre_high - pre_low)
    return interval_to_range(ratio, low, high)


def sample_range(low, high, distribution: rv_continuous, generator: Generator):
    x = distribution.rvs(random_state=generator)

    # full line distribution
    if distribution.a == -inf:
        x = exp(x)

    # lines and half-open intervals
    if distribution.b == inf:
        x = interval_to_range(x, low, high)
        return x

    # the distribution is on a range
    return rescale_range(x, distribution.a, distribution.b, low, high)
