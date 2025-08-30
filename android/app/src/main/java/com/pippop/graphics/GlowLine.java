package com.pippop.graphics;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Point;
import java.nio.FloatBuffer;

public class GlowLine {

  // Each point is 3 float values: {x,y, alphaMultiplier}
  private final FloatBuffer buffer;
  private final FloatBuffer temp;

  public GlowLine() {
    this(300);
  }

  public GlowLine(int size) {
    buffer = Graphics.createFloatBuffer(3 * size);
    temp = Graphics.createFloatBuffer(2 * size);
  }

  public void update(Edge edge, float width) {
    buffer.clear();
    temp.clear();

    Point start = edge.getStart();
    Point end = edge.getEnd();

    edge.flatten(temp);
    temp.put(end.x).put(end.y);
    temp.flip();

    // start cap
    float x2 = temp.get(2);
    float y2 = temp.get(3);
    double capStartAngle = Math.atan2(y2 - start.y, x2 - start.x) - Math.PI / 2;
    for (int i = 0; i < 10; ++i) {
      double angle = capStartAngle - i * Math.PI / 9;
      float dx = width * (float) Math.cos(angle);
      float dy = width * (float) Math.sin(angle);
      buffer.put(start.x + dx).put(start.y + dy).put(0);
      buffer.put(start.x + dx).put(start.y + dy).put(0);
      buffer.put(start.x).put(start.y).put(1);
    }

    // top length
    double px = start.x;
    double py = start.y;
    for (int i = 2; i < temp.limit() - 2; i += 2) {
      float x = temp.get(i);
      float y = temp.get(i + 1);
      float nx = temp.get(i + 2);
      float ny = temp.get(i + 3);

      double normalAngle = (Math.PI / 2) + Math.atan2(ny - py, nx - px);

      buffer
          .put(x + width * (float) Math.cos(normalAngle))
          .put(y + width * (float) Math.sin(normalAngle))
          .put(0);
      buffer.put(x).put(y).put(1);
      px = x;
      py = y;
    }

    capStartAngle = Math.atan2(end.y - py, end.x - px) + Math.PI / 2;
    buffer
        .put(end.x + width * (float) Math.cos(capStartAngle))
        .put(end.y + width * (float) Math.sin(capStartAngle))
        .put(0);
    buffer.put(end.x).put(end.y).put(1);

    // end cap
    for (int i = 0; i < 10; ++i) {
      double angle = capStartAngle - i * Math.PI / 9;
      float dx = width * (float) Math.cos(angle);
      float dy = width * (float) Math.sin(angle);
      buffer.put(end.x + dx).put(end.y + dy).put(0);
      buffer.put(end.x + dx).put(end.y + dy).put(0);
      buffer.put(end.x).put(end.y).put(1);
    }

    // bottom length
    px = end.x;
    py = end.y;
    for (int i = temp.limit() - 2; i > 1; i -= 2) {
      float x = temp.get(i);
      float y = temp.get(i + 1);
      float nx = temp.get(i - 2);
      float ny = temp.get(i - 1);

      double normalAngle = (Math.PI / 2) + Math.atan2(ny - py, nx - px);

      buffer
          .put(x + width * (float) Math.cos(normalAngle))
          .put(y + width * (float) Math.sin(normalAngle))
          .put(0);
      buffer.put(x).put(y).put(1);

      px = x;
      py = y;
    }

    capStartAngle = Math.atan2(py - start.y, px - start.x) - Math.PI / 2;
    buffer
        .put(start.x + width * (float) Math.cos(capStartAngle))
        .put(start.y + width * (float) Math.sin(capStartAngle))
        .put(0);
    buffer.put(start.x).put(start.y).put(1);
    buffer.flip();
  }

  public void update(Bubble bubble, float width) {
    buffer.clear();
    temp.clear();
    Point center = bubble.getCenter();
    Point start = bubble.getFirstEdge().getStart();

    for (Edge edge : bubble) {
      edge.flatten(temp);
    }
    temp.put(start.x).put(start.y).put(1);
    temp.flip();

    for (int i = 0; i < temp.limit() - 2; i += 2) {
      float x = temp.get(i);
      float y = temp.get(i + 1);
      float dx = x - center.x;
      float dy = y - center.y;
      float hypot = (float) (width / Math.hypot(dx, dy));

      dx *= hypot;
      dy *= hypot;
      buffer.put(x).put(y).put(1);
      buffer.put(x + dx).put(y + dy).put(0);
    }
    buffer.flip();
  }

  public FloatBuffer getBuffer() {
    return buffer;
  }
}
