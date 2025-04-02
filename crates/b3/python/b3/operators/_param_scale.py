from scipy.stats import rv_continuous
from math import log

from ._util import sample_range
from b3 import State, Proposal, Parameter


class ParamScale:
    # TODO: upper/lower?
    def __init__(
        self,
        param: Parameter,
        factor: float,
        distribution: rv_continuous,
        dimensions: "one" | "all" | "independent",
        wegith=1,
    ):
        if not 0 < factor < 1:
            raise ValueError(f"factor must be between 0 and 1, got {factor}")
        self.factor = factor
        self.distribution = distribution
        self.dimensions = dimensions

    def propose(self, state: State) -> Proposal:
        generator = state.rng.generator()

        low, high = self.factor, 1 / self.factor
        scale = sample_range(low, high, self.distribution, generator)

        match self.dimensions:
            case "one":
                index = generator.choice(len(self.param))
                if self.param[index] == 0:
                    return Proposal.Reject()
                self.param[index] *= scale

                ratio = -log(scale)
                return Proposal.Hastings(ratio)
            case "all":
                # TODO: overload arithmetic for the whole parameter
                num_scaled = 0
                for i in range(len(self.param)):
                    if self.param[i] != 0:
                        self.param[i] *= scale
                        num_scaled += 1

                # XXX: BEAST2 claims that the Hastings ratio is (num_dimensions
                # - 1) bigger than the 1-parameter case.  The proof should be
                # in a certain Alexei/Nicholes article.  I'll have to
                # investigate, as it's unclear what's supposed to happen when
                # there are only two dimensions (or only two-non zero values).
                ratio = num_scaled * log(scale)
                return Proposal.Hastings(ratio)
            case "independent":
                ratio = 0

                for i in range(len(self.param)):
                    scale = sample_range(low, high, self.distribution, generator)
                    self.param[i] *= scale
                    ratio -= log(scale)

                return Proposal.Hastings(ratio)
