package com.pippop.graphics;

import java.nio.FloatBuffer;

public class PolygonCrossingsEvaluator {

  private static final double distance = Float.MAX_VALUE / 16;
  private static final double epsilon = 1E-7f;

  public static boolean contains(FloatBuffer buffer, float x, float y) {
    boolean inside = false;
    for (int i = 2; i < buffer.limit() - 4; i += 2) {
      float x1 = buffer.get(i);
      float y1 = buffer.get(i + 1);
      float x2 = buffer.get(i + 2);
      float y2 = buffer.get(i + 3);
      if (intersects(x1, y1, x2, y2, x, y)) {
        inside = !inside;
      }
    }
    return inside;
  }

  private static boolean intersects(float x1, float y1, float x2, float y2, float x, float y) {
    if (y1 > y2) {
      return intersects(x2, y2, x1, y1, x, y);
    }

    if (y == y1 || y == y2) {
      y += 0.0001;
    }

    return !(y > y2)
        && !(y < y1)
        && !(x >= Math.max(x1, x2))
        && (x < Math.min(x1, x2) || (y - y1) / (x - x1) >= (y2 - y1) / (x2 - x1));
  }
}
