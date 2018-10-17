package com.pippop.graphics;

import com.pippop.graph.Edge;
import com.pippop.graph.Variable;
import java.nio.FloatBuffer;

class CurveFlattener {

  private static final double FLATNESS = 2;

  private static float ax;
  private static float bx;
  private static float cx;
  private static float dx;

  private static float ay;
  private static float by;
  private static float cy;
  private static float dy;

  public static synchronized void flatten(FloatBuffer buffer, Edge edge) {
    Variable start = edge.getStart();
    Variable startCtrl = edge.getStartCtrl();
    Variable endCtrl = edge.getEndCtrl();
    Variable end = edge.getEnd();

    ax = start.x;
    bx = 3 * (startCtrl.x - start.x);
    cx = 3 * (endCtrl.x - 2 * startCtrl.x + start.x);
    dx = end.x - start.x + 3 * (startCtrl.x - endCtrl.x);

    ay = start.y;
    by = 3 * (startCtrl.y - start.y);
    cy = 3 * (endCtrl.y - 2 * startCtrl.y + start.y);
    dy = end.y - start.y + 3 * (startCtrl.y - endCtrl.y);

    if (!buffer.hasRemaining()) {
      return;
    }
    buffer.put(start.x);
    buffer.put(start.y);

    flatten(0f, start.x, start.y, 1f, end.x, end.y, buffer);
  }

  private static void flatten(
      float low, float lowX, float lowY, float high, float highX, float highY, FloatBuffer buffer) {
    if (!buffer.hasRemaining()) {
      return;
    }
    float mid = (low + high) / 2;
    float midX = getX(mid);
    float midY = getY(mid);

    float approxX = (lowX + highX) / 2;
    float approxY = (lowY + highY) / 2;

    double distance = Math.hypot(midX - approxX, midY - approxY);
    if (distance > FLATNESS) {
      flatten(low, lowX, lowY, mid, midX, midY, buffer);

      if (!buffer.hasRemaining()) {
        return;
      }
      buffer.put(midX);
      buffer.put(midY);

      flatten(mid, midX, midY, high, highX, highY, buffer);
    } else {
      buffer.put(midX);
      buffer.put(midY);
    }
  }

  private static float getX(float t) {
    return ax + t * (bx + t * (cx + t * dx));
  }

  private static float getY(float t) {
    return ay + t * (by + t * (cy + t * dy));
  }
}
