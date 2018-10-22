package com.pippop.graphics;

import com.pippop.graph.Edge;
import com.pippop.graph.Point;
import java.nio.FloatBuffer;

public class GlowLine {

  // Each point is 3 float values: {x,y, alpha}
  private final FloatBuffer buffer = Graphics.createVertexBuffer(300);
  private final FloatBuffer temp = Graphics.createVertexBuffer(100);

  public void update(Edge edge, float width) {
    buffer.clear();

    populateTemp(edge);

    Point start = edge.getStart();
    Point end = edge.getEnd();

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

      double normalAngle = (Math.PI / 2) + Math.atan2((ny - py) / 2, (nx - px) / 2);

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

      double normalAngle = (Math.PI / 2) + Math.atan2((ny - py) / 2, (nx - px) / 2);

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

  private void populateTemp(Edge edge) {
    temp.clear();
    edge.flatten(temp);
    Point end = edge.getEnd();
    temp.put(end.x).put(end.y);
    temp.flip();
  }

  public FloatBuffer getBuffer() {
    return buffer;
  }
}
