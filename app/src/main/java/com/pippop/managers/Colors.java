package com.pippop.managers;

import com.pippop.graphics.Color;
import com.pippop.util.RandomChooser;
import java.util.Arrays;
import java.util.List;

public class Colors {

  private static final Color TURQUOISE = new Color(85 / 255f, 196 / 255f, 200 / 255f, 1);
  private static final Color ROSE = new Color(212 / 255f, 131 / 255f, 145 / 255f, 1);
  private static final Color GREEN = new Color(95 / 255f, 168 / 255f, 69 / 255f, 1);
  private static final Color YELLOW = new Color(192 / 255f, 199 / 255f, 49 / 255f, 1);
  private static final Color RED = new Color(212 / 255f, 31 / 255f, 53 / 255f, 1);
  private static final Color ORANGE = new Color(236 / 255f, 133 / 255f, 35 / 255f, 1);
  private static final Color DARKGRAY = new Color(153 / 255f, 167 / 255f, 138 / 255f, 1);
  private static final Color LIGHTGRAY = new Color(237 / 255f, 216 / 255f, 194 / 255f, 1);

  private static final List<Color> ALL_COLORS =
      Arrays.asList(TURQUOISE, ROSE, GREEN, YELLOW, RED, ORANGE, DARKGRAY, LIGHTGRAY);

  public static RandomChooser<Color> getChooser(int size) {
    if (size > ALL_COLORS.size()) {
      throw new IllegalArgumentException(
          "Cannot allocate more than " + ALL_COLORS.size() + " colors");
    }

    RandomChooser<Color> rc = new RandomChooser<>();
    for (Color color : ALL_COLORS.subList(0, size)) {
      rc.add(color, 1);
    }
    return rc;
  }
}
