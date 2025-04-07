import numpy as np
from b3 import Parameter


class JC:
    def __init__(self):
        self.dimensions = 4

        self.matrix = np.array(
            [
                [-3, 1, 1, 1],
                [1, -3, 1, 1],
                [1, 1, -3, 1],
                [1, 1, 1, -3],
            ],
            dtype=float,
        )

        self.matrix /= -3

    def get_matrix(self):
        return self.matrix


class K80:
    def __init__(self, kappa: Parameter):
        self.dimensions = 4
        self.kappa = kappa

    def get_matrix(self):
        k = self.kappa
        s = np.array(
            [
                [-2 - k, 1, k, 1],
                [1, -2 - k, 1, k],
                [k, 1, -2 - k, 1],
                [1, k, 1, -2 - k],
            ]
        )
        s /= 2 + k

        return s


class F81:
    def __init__(self, frequencies):
        self.dimensions = 4
        a, c, g, t = frequencies
        self.matrix = np.array(
            [
                [a - 1, c, g, t],
                [a, c - 1, g, t],
                [a, c, g - 1, t],
                [a, c, g, t - 1],
            ]
        )
        self.matrix /= 1 - a**2 - c**2 - g**2 - t**2

    def get_matrix(self):
        return self.matrix


class HKY:
    def __init__(self, frequencies, kappa: Parameter):
        self.dimensions = 4
        # XXX: what delta should this use?
        if abs(sum(frequencies)) < 0.01:
            s = sum(frequencies)
            raise ValueError(f"Sum of frequencies must be 1, got {s}")

        if len(kappa) == 0:
            raise ValueError("Expected single-dimensional parameter")

        if not kappa.is_real():
            raise ValueError("Expected a real parameter")

        self.frequencies = frequencies
        self.kappa = kappa

    def get_matrix(self):
        k = self.kappa[0]
        a, c, g, t = self.frequencies
        s = np.array(
            [
                [0, c, k * g, t],
                [a, 0, g, k * t],
                [k * a, c, 0, t],
                [a, k * c, g, 0],
            ]
        )

        for i in range(4):
            s[i, i] = -s[i].sum()

        return s


# TODO: GTR
