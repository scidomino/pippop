package com.pippop.graphics;

import java.util.Arrays;
import java.util.List;

public class Color {

  public static final Color WHITE = new Color(1f, 1f, 1f, 1f);
  public static final Color TRANSPARENT_WHITE = new Color(1f, 1f, 1f, .5f);
  public static final Color BLACK = new Color(0, 0, 0, 1);

  // Game Colors
  private static final Color TURQUOISE = new Color(85 / 255f, 196 / 255f, 200 / 255f, 1);
  private static final Color ROSE = new Color(212 / 255f, 131 / 255f, 145 / 255f, 1);
  private static final Color GREEN = new Color(95 / 255f, 168 / 255f, 69 / 255f, 1);
  private static final Color YELLOW = new Color(192 / 255f, 199 / 255f, 49 / 255f, 1);
  private static final Color RED = new Color(212 / 255f, 31 / 255f, 53 / 255f, 1);
  private static final Color ORANGE = new Color(236 / 255f, 133 / 255f, 35 / 255f, 1);

  private static final List<Color> ALL_COLORS =
      Arrays.asList(TURQUOISE, ROSE, GREEN, YELLOW, RED, ORANGE);

  public final float[] value = new float[4];

  public Color(float r, float g, float b, float a) {
    value[0] = r;
    value[1] = g;
    value[2] = b;
    value[3] = a;
  }

  public static List<Color> getGroup(int size) {
    return ALL_COLORS.subList(0, size);
  }

  public float getRed() {
    return value[0];
  }

  public float getBlue() {
    return value[1];
  }

  public float getGreen() {
    return value[2];
  }

  public float getAlpha() {
    return value[3];
  }

  public Color withAlpha(float alpha) {
    return new Color(value[0], value[1], value[2], alpha);
  }

  @Override
  public String toString() {
    return "Color{" + "value=" + Arrays.toString(value) + '}';
  }
}
