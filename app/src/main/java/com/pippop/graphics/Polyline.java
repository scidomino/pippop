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
      float y = width * (float) Math.sin(normalAngle);
      float x = width * (float) Math.cos(normalAngle);
      vertices.put(x2 - x);
      vertices.put(y2 - y);
      vertices.put(x2 + x);
      vertices.put(y2 + y);

      lastAngle = angle;
    }
    double normal = lastAngle + Math.PI / 2;
    float y = width * (float) Math.sin(normal);
    float x = width * (float) Math.cos(normal);
    vertices.put(x1 - x);
    vertices.put(y1 - y);
    vertices.put(x1 + x);
    vertices.put(y1 + y);

    vertices.flip();

    populateCap(endCap, x1, y1, normal, width);
  }

  private void populateCap(FloatBuffer cap, float cX, float cY, double startAngle, float width) {
    cap.clear();

    cap.put(cX);
    cap.put(cY);

    for (int i = 0; i <= CAP_DIVISIONS; i++) {
      double angle = startAngle + i * Math.PI / CAP_DIVISIONS;
      float y = width * (float) Math.sin(angle);
      float x = width * (float) Math.cos(angle);
      cap.put(cX + x);
      cap.put(cY + y);
    }

    cap.flip();
  }

  private void populateTemp(Edge edge) {
    temp.clear();
    CurveFlattener.flatten(temp, edge);
    Vertex end = edge.getEnd();
    temp.put(end.x);
    temp.put(end.y);
    temp.flip();
  }

  public void addPoint(float x, float y) {
    vertices.put(x);
    vertices.put(y);
  }

  public void done() {
    vertices.flip();
  }

  public FloatBuffer getVertices() {
    return vertices;
  }

  public FloatBuffer getStartCap() {
    return startCap;
  }

  public FloatBuffer getEndCap() {
    return endCap;
  }
}
