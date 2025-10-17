import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from scipy.integrate import quad
from matplotlib.widgets import Slider
from sympy import *
import numpy as np

# Define symbols for the quadratic rational Bezier curve
P = Matrix([[symbols("P_" + str(p) + "_" + n) for n in ["x", "y"]] for p in [0, 1, 2]])
w = Matrix(symarray("w", (3)))  # the 3 weights
t = symbols("t")

# The quadratic Bernstein basis polynomials
B = Matrix([(1 - t) ** 2, 2 * t * (1 - t), t**2])

# The parametric curve
C = P.T * diag(*w) * B / B.dot(w)


# change of vars.
# d = distance between endpoints
# k = area
# n = eccentricity
d, k, n = symbols("d, k, n")

C = C.subs(
    {
        "w_0": 1,
        "w_1": n / sqrt(1 + n**2),
        "w_2": 1,
        "P_0_x": -d,
        "P_0_y": 0,
        "P_1_x": 0,
        "P_1_y": k / (d * ((n**3 + n) * atan2(1, n) - n**2)),
        "P_2_x": d,
        "P_2_y": 0,
    }
)

print(C)

x = C[0]
y = C[1]

# Lambdify the curve expressions for numerical evaluation
x_func = lambdify((t, n, k, d), x)
y_func = lambdify((t, n, k, d), y)


# Create a figure and axes for the plot
fig, ax = plt.subplots()
plt.subplots_adjust(left=0.1, bottom=0.35)

# Initial values for sliders
n_init = 1.0
k_init = 1.0
d_init = 1.0

# Create a t-space for plotting
t_vals = np.linspace(0, 1, 200)


# Plot the initial curve
x_vals = x_func(t_vals, n_init, k_init, d_init)
y_vals = y_func(t_vals, n_init, k_init, d_init)
(line,) = ax.plot(x_vals, y_vals, lw=2)
ax.set_xlabel("x")
ax.set_ylabel("y")
ax.set_title("Quadratic Rational Bezier Curve")
ax.grid(True)
ax.axis("equal")

# Add text for d*k and area
dk_text = ax.text(0.05, 0.95, "", transform=ax.transAxes, verticalalignment="top")
area_text = ax.text(0.05, 0.90, "", transform=ax.transAxes, verticalalignment="top")

# Add sliders for n, k, and d
axcolor = "lightgoldenrodyellow"
ax_n = plt.axes([0.1, 0.2, 0.8, 0.03], facecolor=axcolor)
ax_k = plt.axes([0.1, 0.15, 0.8, 0.03], facecolor=axcolor)
ax_d = plt.axes([0.1, 0.1, 0.8, 0.03], facecolor=axcolor)

s_n = Slider(ax_n, "n", -10.0, 10.0, valinit=n_init)
s_k = Slider(ax_k, "k", -10.0, 10.0, valinit=k_init)
s_d = Slider(ax_d, "d", 0.1, 10.0, valinit=d_init)


# Update function for sliders
def update(val):
    n_val = s_n.val
    k_val = s_k.val
    d_val = s_d.val
    x_vals = x_func(t_vals, n_val, k_val, d_val)
    y_vals = y_func(t_vals, n_val, k_val, d_val)
    line.set_xdata(x_vals)
    line.set_ydata(y_vals)

    ax.relim()
    ax.autoscale_view()
    fig.canvas.draw_idle()


# Initial update
update(0)

s_n.on_changed(update)
s_k.on_changed(update)
s_d.on_changed(update)

plt.show()
