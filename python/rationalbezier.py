import numpy as np
import matplotlib.pyplot as plt
from matplotlib.widgets import Slider


# Define the rational Bezier curve function
def rational_bezier(p0, p1, p2, w1, t):
    """Calculates a point on a rational quadratic Bezier curve."""
    p0 = np.array(p0)
    p1 = np.array(p1)
    p2 = np.array(p2)

    # The numerator of the rational Bézier curve formula
    numerator = (
        np.outer((1 - t) ** 2, p0)
        + np.outer(2 * (1 - t) * t, p1) * w1
        + np.outer(t**2, p2)
    )

    # The denominator of the rational Bézier curve formula
    denominator = ((1 - t) ** 2) + (w1 * 2 * (1 - t) * t) + (t**2)

    # Return the points on the curve
    return numerator / denominator[:, np.newaxis]


class PointManager:
    """A manager for draggable points on a matplotlib plot."""

    def __init__(self, points, update_func):
        self.points = points
        self.update_func = update_func
        self.active_point = None
        self.press_info = None

    def connect(self):
        "connect to all the events we need"
        self.cidpress = self.points[0].figure.canvas.mpl_connect(
            "button_press_event", self.on_press
        )
        self.cidrelease = self.points[0].figure.canvas.mpl_connect(
            "button_release_event", self.on_release
        )
        self.cidmotion = self.points[0].figure.canvas.mpl_connect(
            "motion_notify_event", self.on_motion
        )

    def on_press(self, event):
        "on button press, find the point clicked and store it"
        if event.inaxes not in [p.axes for p in self.points]:
            return

        for point in self.points:
            contains, attrd = point.contains(event)
            if contains:
                self.active_point = point
                x, y = point.get_xdata(), point.get_ydata()
                self.press_info = (x[0], y[0]), event.xdata, event.ydata
                return

    def on_motion(self, event):
        "on motion, move the active point"
        if self.active_point is None:
            return
        if event.inaxes != self.active_point.axes:
            return

        (x0, y0), xpress, ypress = self.press_info
        dx = event.xdata - xpress
        dy = event.ydata - ypress
        self.active_point.set_data([x0 + dx], [y0 + dy])

        self.update_func()
        self.active_point.figure.canvas.draw_idle()

    def on_release(self, event):
        "on release, clear the active point"
        self.active_point = None
        self.press_info = None

    def disconnect(self):
        "disconnect all the stored connection ids"
        self.points[0].figure.canvas.mpl_disconnect(self.cidpress)
        self.points[0].figure.canvas.mpl_disconnect(self.cidrelease)
        self.points[0].figure.canvas.mpl_disconnect(self.cidmotion)


# --- Main Setup ---
fig, ax = plt.subplots(figsize=(8, 8))
plt.subplots_adjust(left=0.1, bottom=0.3)
ax.set_xlim(0, 1)
ax.set_ylim(0, 1)
ax.set_aspect("equal", adjustable="box")
ax.set_title("Rational Quadratic Bézier Curve - Drag Points and Use Sliders")

# Initial control points
control_points_coords = [[0.1, 0.2], [0.5, 0.5], [0.9, 0.2]]
control_point_plots = [
    ax.plot(p[0], p[1], "ro", markersize=10)[0] for p in control_points_coords
]

# Initial values
initial_w1 = 1.0
initial_p1_pos = 0.5

# The control polygon
(control_polygon,) = ax.plot(
    [p[0] for p in control_points_coords],
    [p[1] for p in control_points_coords],
    "r--",
    zorder=1,
)

# The Bézier curve
t = np.linspace(0, 1, 200)
curve_points = rational_bezier(
    control_points_coords[0],
    control_points_coords[1],
    control_points_coords[2],
    initial_w1,
    t,
)
(bezier_curve,) = ax.plot(
    curve_points[:, 0], curve_points[:, 1], "b-", linewidth=2, zorder=2
)

# --- Interactivity ---


# Update function
def update_plot(val=None):
    # Update P0 and P2 coordinates from their plot objects
    p0_coords = np.array([control_point_plots[0].get_xdata()[0], control_point_plots[0].get_ydata()[0]])
    p2_coords = np.array([control_point_plots[2].get_xdata()[0], control_point_plots[2].get_ydata()[0]])
    control_points_coords[0] = list(p0_coords)
    control_points_coords[2] = list(p2_coords)

    # Calculate P1's position on the perpendicular bisector
    midpoint = (p0_coords + p2_coords) / 2.0
    v = p2_coords - p0_coords
    perpendicular_v = np.array([-v[1], v[0]])
    
    # Normalize the perpendicular vector to give the slider a consistent feel
    dist = np.linalg.norm(v)
    if dist > 0:
        perpendicular_v = perpendicular_v / np.linalg.norm(perpendicular_v) * (dist / 2.0)

    t_p1 = slider_p1_pos.val
    p1_coords = midpoint + t_p1 * perpendicular_v
    control_points_coords[1] = list(p1_coords)
    control_point_plots[1].set_data([p1_coords[0]], [p1_coords[1]])

    # Update control polygon
    control_polygon.set_xdata([p[0] for p in control_points_coords])
    control_polygon.set_ydata([p[1] for p in control_points_coords])

    # Update curve
    w1 = slider_w1.val
    new_curve_points = rational_bezier(
        control_points_coords[0],
        control_points_coords[1],
        control_points_coords[2],
        w1,
        t,
    )
    bezier_curve.set_xdata(new_curve_points[:, 0])
    bezier_curve.set_ydata(new_curve_points[:, 1])

    fig.canvas.draw_idle()


# Draggable points manager (only for start and end points)
point_manager = PointManager([control_point_plots[0], control_point_plots[2]], update_plot)
point_manager.connect()

# Slider for the weight w1
ax_w1 = plt.axes([0.1, 0.15, 0.8, 0.03])
slider_w1 = Slider(ax_w1, "Weight w1", -1.0, 1.0, valinit=initial_w1)
slider_w1.on_changed(update_plot)

# Slider for P1 position
ax_p1_pos = plt.axes([0.1, 0.1, 0.8, 0.03])
slider_p1_pos = Slider(ax_p1_pos, "P1 Position", -10.0, 10.0, valinit=initial_p1_pos)
slider_p1_pos.on_changed(update_plot)

# Initial plot update
update_plot()

plt.show()
