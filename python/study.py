import matplotlib.pyplot as plt
from matplotlib.widgets import Slider
from sympy import *
import numpy as np
from scipy.integrate import quad

# Define symbols for the quadratic rational Bezier curve
P = Matrix([[symbols("P_" + str(p) + "_" + n) for n in ["x", "y"]] for p in [0, 1, 2]])
w = Matrix(symarray("w", (3)))  # the 3 weights
t = symbols("t")

# The quadratic Bernstein basis polynomials
B = Matrix([(1 - t) ** 2, 2 * t * (1 - t), t**2])

# The parametric curve
C_symbolic = P.T * diag(*w) * B / B.dot(w)

# reparameterize 
u, v = symbols("u, v")

l = u + v
k = u - v

# Weird factor required to make area = k * ||P_2-P_0||^2
weirdy_factor = 2 * k / ((l**3 + l) * atan2(1, l) - l**2)

C_symbolic = C_symbolic.subs(
    {
        w[0]: 1,
        w[1]: l / sqrt(1 + l**2),
        w[2]: 1,
        # Constrain P_1 to be on the line equidistant from P_0 and P_2, parameterized by k.
        P[1, 0]: (P[0, 0] + P[2, 0]) / 2 + weirdy_factor * (P[0, 1] - P[2, 1]),
        P[1, 1]: (P[0, 1] + P[2, 1]) / 2 + weirdy_factor * (P[2, 0] - P[0, 0]),
    }
)

area = k * ((P[0, 0] - P[2, 0]) ** 2 + (P[0, 1] - P[2, 1]) ** 2)
