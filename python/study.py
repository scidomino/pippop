from sympy import *
from sympy.integrals.rationaltools import ratint

# Define symbols for the quadratic rational Bezier curve
P = Matrix([[symbols("P_" + str(p) + "_" + n) for n in ["x", "y"]] for p in [0, 1, 2]])
w = Matrix(symarray("w", (3)))  # the 3 weights
t, n, k, d = symbols("t, n, k, d")

# The quadratic Bernstein basis polynomials
B = Matrix([(1 - t) ** 2, 2 * t * (1 - t), t**2])

# The parametric curve
C = P.T * diag(*w) * B / B.dot(w)

continuous_acot = Piecewise((acot(n), n > 0), (acot(n) + pi, n < 0), (pi / 2, True))
C = C.subs(
    {
        "w_0": 1,
        "w_1": n / sqrt(1 + n**2),
        "w_2": 1,
        "P_0_x": -d,
        "P_0_y": 0,
        "P_1_x": 0,
        "P_1_y": k / (n**2 - (n**3 + n) * continuous_acot),
        "P_2_x": d,
        "P_2_y": 0,
    }
)
pprint(C)


x = C[0]
y = C[1]
area = simplify(ratint((x * diff(y, t) - y * diff(x, t)), t) / 2)
pprint(area)
