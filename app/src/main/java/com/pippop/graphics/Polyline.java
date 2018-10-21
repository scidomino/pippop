package com.pippop.graphics;

import com.pippop.graph.Edge;
import com.pippop.graph.Vertex;
import java.nio.FloatBuffer;

public class Polyline {
  private static final int CAP_DIVISIONS = 10;

  private final FloatBuffer vertices;
  private final FloatBuffer startCap;
  private final FloatBuffer endCap;
  private final FloatBuffer temp;

  public Polyline(int maxVertices) {
    vertices = Graphics.createVertexBuffer(2 * maxVertices);
    temp = Graphics.createVertexBuffer(maxVertices);
    startCap = Graphics.createVertexBuffer(CAP_DIVISIONS + 2);
    endCap = Graphics.createVertexBuffer(CAP_DIVISIONS + 2);
  }

  public void update(Edge edge, float width) {
    populateTemp(edge);

    vertices.clear();

    float x2 = 2 * temp.get(0) - temp.get(2);
    float y2 = 2 * temp.get(1) - temp.get(3);
    float x1 = temp.get(0);
    float y1 = temp.get(1);
    temp.position(2);
    double lastAngle = Math.atan2(y2 - y1, x2 - x1);

    populateCap(startCap, x1, y1, lastAngle - Math.PI / 2, width);

    while (temp.hasRemaining()) {
      x2 = x1;
      y2 = y1;
      x1 = temp.get();
      y1 = temp.get();
      double angle = Math.atan2(y2 - y1, x2 - x1);

      double normalAngle = (Math.PI + lastAngle + angle) / 2;
      float dx = width * (float) Math.cos(normalAngle);
      float dy = width * (float) Math.sin(normalAngle);
      vertices.put(x2 - dx).put(y2 - dy);
      vertices.put(x2 + dx).put(y2 + dy);

      lastAngle = angle;
    }
    double normal = lastAngle + Math.PI / 2;
    float dx = width * (float) Math.cos(normal);
    float dy = width * (float) Math.sin(normal);
    vertices.put(x1 - dx).put(y1 - dy);
    vertices.put(x1 + dx).put(y1 + dy);

    vertices.flip();

    populateCap(endCap, x1, y1, normal, width);
  }

  private void populateCap(FloatBuffer cap, float cX, float cY, double startAngle, float width) {
    cap.clear();
    cap.put(cX).put(cY);
    for (int i = 0; i <= CAP_DIVISIONS; i++) {
      double angle = startAngle + i * Math.PI / CAP_DIVISIONS;
      float y = width * (float) Math.sin(angle);
      float x = width * (float) Math.cos(angle);
      cap.put(cX + x).put(cY + y);
    }
    cap.flip();
  }

  private void populateTemp(Edge edge) {
    temp.clear();
    edge.flatten(temp);
    Vertex end = edge.getEnd();
    temp.put(end.x).put(end.y);
    temp.flip();
  }

  FloatBuffer getVertices() {
    return vertices;
  }

  FloatBuffer getStartCap() {
    return startCap;
  }

  FloatBuffer getEndCap() {
    return endCap;
  }
}
