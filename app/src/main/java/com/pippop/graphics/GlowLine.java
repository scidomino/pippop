package com.pippop.graphics;

import com.pippop.graph.Bubble;
import com.pippop.graph.Edge;
import com.pippop.graph.Point;
import java.nio.FloatBuffer;

public class GlowLine {

  // Each point is 3 float values: {x,y, alpha}
  private final FloatBuffer buffer;
  private final FloatBuffer temp;

  public GlowLine() {
    this(100);
  }

  public GlowLine(int size) {
    buffer = Graphics.createVertexBuffer(3 * size);
    temp = Graphics.createVertexBuffer(2 * size);
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

    Point start = bubble.getFirstEdge().getStart();

    for (Edge edge : bubble) {
      edge.flatten(temp);
    }
    for (int i = 0; i < 6; i++) {
      temp.put(temp.get(i));
    }
    temp.flip();

    buffer.put(start.x).put(start.y).put(1);

    float px = start.x;
    float py = start.y;
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
  }

  public FloatBuffer getBuffer() {
    return buffer;
  }
}
