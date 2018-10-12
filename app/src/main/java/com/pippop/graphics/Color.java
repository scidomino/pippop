package com.pippop.graphics;

public class Color {

  public static final Color WHITE = new Color(1f, 1f, 1f, 1f);
  public static final Color GREEN = new Color(0.63671875f, 0.76953125f, 0.22265625f, 1.0f);
  public static final Color BLUE = new Color(1f, 1f, 0f, 1.0f);
  public static final Color RED = new Color(1f, 0f, 0f, 1.0f);
  public static final Color TRANSPARENT_WHITE = new Color(1f, 1f, 1f, .2f);

  final float[] value = new float[4];

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
}
