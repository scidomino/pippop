# A file to play around with sympy

import sympy
import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from scipy.integrate import quad
from matplotlib.widgets import Slider


def positive_acot(x_val):
    """
    Returns the arccotangent of x with a range of (0, pi),
    making it continuous and always positive.
    """
    return sympy.Piecewise(
        (sympy.acot(x_val) + sympy.pi, x_val < 0), (sympy.acot(x_val), True)
    )


# In this parametric equation, m is the area.
n, m, t = sympy.symbols("n m t")

w = n / sympy.sqrt(n**2 + 1)
k = m / (sympy.sqrt(n**2 + 1) * (n**2 + 1 * positive_acot(n) - n))

denominator = (1 - t) ** 2 + w * 2 * t * (1 - t) + t**2

x = (k * 2 * t * (1 - t)) / denominator
y = (1 - 2 * t) / denominator


# Integrand for arc length
integrand = sympy.sqrt(sympy.diff(x, t) ** 2 + sympy.diff(y, t) ** 2)


# Function to calculate arc length for given n and m
def arc_length(n_val, m_val):
    """Calculates the arc length for given values of n and m."""
    f = sympy.lambdify(t, integrand.subs({n: n_val, m: m_val}), "numpy")
    value, _ = quad(f, 0, 1)
    return value


def plot_curve():
    # Lambdify the symbolic expressions for x and y
    x_func = sympy.lambdify((t, n, m), x, "numpy")
    y_func = sympy.lambdify((t, n, m), y, "numpy")

    # Generate t values
    t_vals = np.linspace(0, 1, 400)

    # Initial values
    n_initial = 1.0
    m_initial = 1.0

    # Create the plot
    fig, ax = plt.subplots()
    plt.subplots_adjust(left=0.1, bottom=0.25)

    # Initial plot
    x_vals = x_func(t_vals, n_initial, m_initial)
    y_vals = y_func(t_vals, n_initial, m_initial)
    (line,) = ax.plot(x_vals, y_vals)
    initial_arc_length = arc_length(n_initial, m_initial)
    ax.set_title(
        f"n={n_initial:.2f}, m={m_initial:.2f}, arc length={initial_arc_length:.2f}"
    )
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    ax.grid(True)
    ax.set_xlim(-5, 5)
    ax.set_ylim(-5, 5)
    ax.set_aspect("equal", adjustable="box")

    # Create slider axes
    ax_n = plt.axes([0.1, 0.1, 0.8, 0.03], facecolor="lightgoldenrodyellow")
    ax_m = plt.axes([0.1, 0.05, 0.8, 0.03], facecolor="lightgoldenrodyellow")

    # Create sliders
    slider_n = Slider(ax_n, "n", -5.0, 5.0, valinit=n_initial)
    slider_m = Slider(ax_m, "m", -10.0, 10.0, valinit=m_initial)

    # Update function
    def update(val):
        n_val = slider_n.val
        m_val = slider_m.val
        x_vals = x_func(t_vals, n_val, m_val)
        y_vals = y_func(t_vals, n_val, m_val)
        line.set_xdata(x_vals)
        line.set_ydata(y_vals)
        current_arc_length = arc_length(n_val, m_val)
        ax.set_title(
            f"n={n_val:.2f}, m={m_val:.2f}, arc length={current_arc_length:.2f}"
        )
        fig.canvas.draw_idle()

    # Register update function
    slider_n.on_changed(update)
    slider_m.on_changed(update)

    plt.show()


# # --- 3D Plotting ---

# # Define ranges for n and m


def plot_arc_length():
    # Create a grid of n and m values
    N, M = np.meshgrid(np.linspace(-3, 3, 20), np.linspace(-10, 10, 20))

    # Calculate arc length for each (n, m) pair
    value = np.zeros_like(N, dtype=float)
    for i in range(N.shape[0]):
        for j in range(N.shape[1]):
            value[i, j] = arc_length(N[i, j], M[i, j])

    # Create the 3D plot
    fig = plt.figure()
    ax = fig.add_subplot(111, projection="3d")
    ax.plot_surface(N, M, value, cmap="viridis")

    # Set labels
    ax.set_xlabel("n")
    ax.set_ylabel("m")
    ax.set_zlabel("Arc Length")
    ax.set_title("Arc Length as a function of n and m")

    # Show the plot
    plt.show()


# Print the symbolic equations
print("w =", w)
print("k =", k)
print("x =", x)
print("y =", y)

# plot_arc_length()
plot_curve()
