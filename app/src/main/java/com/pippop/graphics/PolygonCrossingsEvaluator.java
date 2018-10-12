package com.pippop.graphics;

import java.nio.FloatBuffer;

public class PolygonCrossingsEvaluator {

  public static int evaluateCrossings(float x, float y, float distance, FloatBuffer buffer) {
    float x0;
    float x1;
    float y0;
    float y1;
    float epsilon = 0.0f;
    int crossings = 0;

    /* Get a value which is small but not insignificant relative the path. */
    epsilon = 1E-7f;

    x0 = buffer.get(0) - x;
    y0 = buffer.get(1) - y;
    for (int i = 2; i < buffer.limit(); i += 2) {
      x1 = buffer.get(i) - x;
      y1 = buffer.get(i + 1) - y;

      if (y0 == 0.0) y0 -= epsilon;
      if (y1 == 0.0) y1 -= epsilon;
      if (y0 * y1 < 0) if (linesIntersect(x0, y0, x1, y1, epsilon, 0.0, distance, 0.0)) ++crossings;

      x0 = buffer.get(i) - x;
      y0 = buffer.get(i + 1) - y;
    }

    // end segment
    x1 = buffer.get(0) - x;
    y1 = buffer.get(1) - y;
    if (y0 == 0.0) y0 -= epsilon;
    if (y1 == 0.0) y1 -= epsilon;
    if (y0 * y1 < 0) if (linesIntersect(x0, y0, x1, y1, epsilon, 0.0, distance, 0.0)) ++crossings;

    return crossings;
  }

  private static boolean linesIntersect(
      double x1, double y1, double x2, double y2, double x3, double y3, double x4, double y4) {
    double a1, a2, a3, a4;

    // deal with special cases
    if ((a1 = area2(x1, y1, x2, y2, x3, y3)) == 0.0) {
      // check if p3 is between p1 and p2 OR
      // p4 is collinear also AND either between p1 and p2 OR at opposite
      // ends
      if (between(x1, y1, x2, y2, x3, y3)) {
        return true;
      } else {
        if (area2(x1, y1, x2, y2, x4, y4) == 0.0) {
          return between(x3, y3, x4, y4, x1, y1) || between(x3, y3, x4, y4, x2, y2);
        } else {
          return false;
        }
      }
    } else if ((a2 = area2(x1, y1, x2, y2, x4, y4)) == 0.0) {
      // check if p4 is between p1 and p2 (we already know p3 is not
      // collinear)
      return between(x1, y1, x2, y2, x4, y4);
    }

    if ((a3 = area2(x3, y3, x4, y4, x1, y1)) == 0.0) {
      // check if p1 is between p3 and p4 OR
      // p2 is collinear also AND either between p1 and p2 OR at opposite
      // ends
      if (between(x3, y3, x4, y4, x1, y1)) {
        return true;
      } else {
        if (area2(x3, y3, x4, y4, x2, y2) == 0.0) {
          return between(x1, y1, x2, y2, x3, y3) || between(x1, y1, x2, y2, x4, y4);
        } else {
          return false;
        }
      }
    } else if ((a4 = area2(x3, y3, x4, y4, x2, y2)) == 0.0) {
      // check if p2 is between p3 and p4 (we already know p1 is not
      // collinear)
      return between(x3, y3, x4, y4, x2, y2);
    } else { // test for regular intersection
      return ((a1 > 0.0) ^ (a2 > 0.0)) && ((a3 > 0.0) ^ (a4 > 0.0));
    }
  }

  private static double area2(double x1, double y1, double x2, double y2, double x3, double y3) {
    return (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1);
  }

  private static boolean between(double x1, double y1, double x2, double y2, double x3, double y3) {
    if (x1 != x2) {
      return (x1 <= x3 && x3 <= x2) || (x1 >= x3 && x3 >= x2);
    } else {
      return (y1 <= y3 && y3 <= y2) || (y1 >= y3 && y3 >= y2);
    }
  }
}
