package com.pippop.graphics;

import java.util.Arrays;

public class Color {

  public static final Color WHITE = new Color(1f, 1f, 1f, 1f);
  public static final Color TRANSPARENT_WHITE = new Color(1f, 1f, 1f, .5f);
  public static final Color BLACK = new Color(0, 0, 0, 1);

  public final float[] value = new float[4];

  public Color(float r, float g, float b, float a) {
    value[0] = r;
    value[1] = g;
    value[2] = b;
    value[3] = a;
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
