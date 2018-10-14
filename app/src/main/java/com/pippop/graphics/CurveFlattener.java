package com.pippop.graphics;

import com.pippop.graph.Edge;
import com.pippop.graph.Variable;
import java.nio.FloatBuffer;

class CurveFlattener {

  private static final double FLATNESS = 2;

  static void flatten(FloatBuffer buffer, Edge edge) {
    if (!buffer.hasRemaining()) {
      return;
    }

    Variable start = edge.getStart();
    Variable startCtrl = edge.getStartCtrl();
    Variable endCtrl = edge.getEndCtrl();
    Variable end = edge.getEnd();

    buffer.put(start.x);
    buffer.put(start.y);

    Bezier x = new Bezier(start.x, startCtrl.x, endCtrl.x, end.x);
    Bezier y = new Bezier(start.y, startCtrl.y, endCtrl.y, end.y);

    flatten(x, y, 0f, start.x, start.y, 1f, end.x, end.y, buffer);
  }

  private static void flatten(
      Bezier x,
      Bezier y,
      float low,
      float lowX,
      float lowY,
      float high,
      float highX,
      float highY,
      FloatBuffer buffer) {
    if (!buffer.hasRemaining()) {
      return;
    }
    float mid = (low + high) / 2;
    float midX = x.getValue(mid);
    float midY = y.getValue(mid);

    float approxX = (lowX + highX) / 2;
    float approxY = (lowY + highY) / 2;

    double distance = Math.hypot(midX - approxX, midY - approxY);
    if (distance > FLATNESS) {
      flatten(x, y, low, lowX, lowY, mid, midX, midY, buffer);

      if (!buffer.hasRemaining()) {
        return;
      }
      buffer.put(midX);
      buffer.put(midY);

      flatten(x, y, mid, midX, midY, high, highX, highY, buffer);
    } else {
      buffer.put(midX);
      buffer.put(midY);
    }
  }

  private static class Bezier {

    private final float a;
    private final float b;
    private final float c;
    private final float d;

    Bezier(float start, float startCtrl, float endCtrl, float end) {
      a = start;
      b = 3 * (startCtrl - start);
      c = 3 * (endCtrl - 2 * startCtrl + start);
      d = end - start + 3 * (startCtrl - endCtrl);
    }

    private float getValue(float t) {
      return a + t * (b + t * (c + t * d));
    }

    private float getDerivative(float t) {
      return (b + t * (2 * c + 3 * t * d));
    }
  }
}
