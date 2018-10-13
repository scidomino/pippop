package com.pippop.graphics;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Point;
import java.nio.BufferOverflowException;
import java.nio.FloatBuffer;

/**
 * A special collection of points defining a polygon. First point is the center, the last point is
 * the same as the second point.
 */
public class Polygon {

  private static final float BIG_VALUE = java.lang.Float.MAX_VALUE / 10.0f;

  private FloatBuffer vertices;

  public Polygon(int maxVertices) {
    vertices = Graphics.createVertexBuffer(maxVertices);
  }

  public FloatBuffer getVertices() {
    return vertices;
  }

  public void update(Bubble bubble) {
    try {
      vertices.clear();
      vertices.put(bubble.getCenter().x);
      vertices.put(bubble.getCenter().y);
      for (Edge edge : bubble) {
        CurveFlattener.flatten(vertices, edge);
      }

      vertices.put(vertices.get(2));
      vertices.put(vertices.get(3));
      vertices.flip();
    } catch (BufferOverflowException e) {
      if (vertices.capacity() > 10000) {
        // Forget it, Jake. It's Chinatown.
        return;
      }
      vertices = Graphics.createVertexBuffer(2 * vertices.capacity());
      update(bubble);
    }
  }

  public void rotate(Point center, double angle, Polygon inPolygon) {
    float sin = (float) Math.sin(angle);
    float cos = (float) Math.cos(angle);

    FloatBuffer in = inPolygon.vertices;
    vertices.clear();
    for (int i = 0; i < in.limit() / 2; i++) {
      int xIndex = i * 2;
      int yIndex = xIndex + 1;
      float x = in.get(xIndex);
      float y = in.get(yIndex);
      float rotatedX = cos * (x - center.x) - sin * (y - center.y) + center.x;
      float rotatedY = sin * (x - center.x) + cos * (y - center.y) + center.y;

      vertices.put(rotatedX);
      vertices.put(rotatedY);
    }

    vertices.put(vertices.get(2));
    vertices.put(vertices.get(3));
    vertices.flip();
  }

  public boolean contains(float x, float y) {
    return ((PolygonCrossingsEvaluator.evaluateCrossings(x, y, BIG_VALUE, vertices) & 1) != 0);
  }
}
