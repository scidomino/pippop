package com.pippop.physics;

import com.pippop.graph.Edge;
import com.pippop.graph.Graph;
import com.pippop.graph.Point;
import java.util.Arrays;

class ForceCalculator {

  private static final float SURFACE_TENSION = .3f;
  private static final float PRESSURE_TENSION = .04f;
  private static final float GRAVITY = .001f;
  private static final float PRESSURE_SPEED_BUMP = 2;

  private final Force surfaceX = new SurfaceForce.X();
  private final Force surfaceY = new SurfaceForce.Y();
  private final Force pressureX = new PressureForce.X();
  private final Force pressureY = new PressureForce.Y();

  void calculateForceX(float[] vertexForce, float[] ctrlPointForce, Graph graph, float centerX) {
    Point bubbleCenter = graph.getOpenAir().getCenter();
    float gravityX = GRAVITY * (centerX - bubbleCenter.x);
    calculateForce(vertexForce, ctrlPointForce, surfaceX, pressureX, gravityX, graph);
  }

  void calculateForceY(float[] vertexForce, float[] ctrlPointForce, Graph graph, float centerY) {
    Point bubbleCenter = graph.getOpenAir().getCenter();
    float gravityY = GRAVITY * (centerY - bubbleCenter.y);
    calculateForce(vertexForce, ctrlPointForce, surfaceY, pressureY, gravityY, graph);
  }

  private void calculateForce(
      float[] vertexForce,
      float[] ctrlPointForce,
      Force surface,
      Force pressure,
      float gravity,
      Graph graph) {
    Arrays.fill(vertexForce, 0, graph.getVertices().size(), 0f);
    for (Edge edge : graph.getEdges()) {
      double pressureDiff = edge.getPressure(PRESSURE_SPEED_BUMP);

      float vForce = gravity;
      vForce -= SURFACE_TENSION * surface.getVertex(edge);
      vForce -= PRESSURE_TENSION * pressureDiff * pressure.getVertex(edge);
      vertexForce[edge.getStart().getIndex()] += vForce;

      float cpForce = gravity;
      cpForce -= SURFACE_TENSION * surface.getCtrl(edge);
      cpForce -= PRESSURE_TENSION * pressureDiff * pressure.getCtrl(edge);
      ctrlPointForce[edge.getStartCtrl().getIndex()] = cpForce;
    }
  }
}
