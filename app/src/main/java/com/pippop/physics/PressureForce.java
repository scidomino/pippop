package com.pippop.physics;

import com.pippop.graph.Edge;

class PressureForce {
  private static final float ONE_TWENTIETH = 1f / 20f;

  public static class X implements Force {

    @Override
    public float getVertex(Edge edge) {
      float sy = edge.getStart().y;
      float scy = edge.getStartCtrl().y;
      float ecy = edge.getEndCtrl().y;
      float ey = edge.getEnd().y;

      return (-10 * sy - 6 * scy - 3 * ecy - ey) * ONE_TWENTIETH;
    }

    @Override
    public float getCtrl(Edge edge) {
      float sy = edge.getStart().y;
      float ecy = edge.getEndCtrl().y;
      float ey = edge.getEnd().y;
      return (6 * sy - 3 * ecy - 3 * ey) * ONE_TWENTIETH;
    }
  }

  public static class Y implements Force {

    @Override
    public float getVertex(Edge edge) {
      float sx = edge.getStart().x;
      float scx = edge.getStartCtrl().x;
      float ecx = edge.getEndCtrl().x;
      float ex = edge.getEnd().x;
      return (-10 * sx + 6 * scx + 3 * ecx + ex) * ONE_TWENTIETH;
    }

    @Override
    public float getCtrl(Edge edge) {
      float sx = edge.getStart().x;
      float ecx = edge.getEndCtrl().x;
      float ex = edge.getEnd().x;
      return (-6 * sx + 3 * ecx + 3 * ex) * ONE_TWENTIETH;
    }
  }
}
