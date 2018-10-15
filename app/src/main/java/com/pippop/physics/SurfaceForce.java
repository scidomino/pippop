package com.pippop.physics;

import com.pippop.graph.Edge;
import com.pippop.graph.Variable;

class SurfaceForce {

  private static final float[] ABSCISSAS_3 = {
      (float) (.5 * (1 - Math.sqrt(3.0 / 5.0))), .5f, (float) (.5 * (1 + Math.sqrt(3.0 / 5.0)))
  };
  private static final float[] WEIGHTS_3 = {5f / 18f, 4f / 9f, 5f / 18f};

  private static FloatFunction derStartX(
      float sx, float sy, float scx, float scy, float ecx, float ecy, float ex, float ey) {
    float ax = 3 * (scx - ecx) + ex - sx;
    float ay = 3 * (scy - ecy) + ey - sy;
    float bx = 2 * (sx - 2 * scx + ecx);
    float by = 2 * (sy - 2 * scy + ecy);
    float cx = scx - sx;
    float cy = scy - sy;

    return p -> {
      float bezierXDp = cx + p * (bx + p * ax);
      float bezierYDp = cy + p * (by + p * ay);

      float hypot = (float) Math.hypot(bezierXDp, bezierYDp);
      if (hypot == 0) {
        return 0;
      }
      float bezierYDpDStartX = 3 * p * p;
      return bezierYDpDStartX * bezierXDp / hypot;
    };
  }

  private static FloatFunction derStartCtrlX(
      float sx, float sy, float scx, float scy, float ecx, float ecy, float ex, float ey) {
    float ax = 3 * (scx - ecx) + ex - sx;
    float ay = 3 * (scy - ecy) + ey - sy;
    float bx = 2 * (sx - 2 * scx + ecx);
    float by = 2 * (sy - 2 * scy + ecy);
    float cx = scx - sx;
    float cy = scy - sy;

    return p -> {
      float bezierXDp = cx + p * (bx + p * ax);
      float bezierYDp = cy + p * (by + p * ay);

      float hypot = (float) Math.hypot(bezierXDp, bezierYDp);
      if (hypot == 0) {
        return 0;
      }
      float bezierYDpDStartCtrlX = p * (6 - 9 * p);
      return bezierYDpDStartCtrlX * bezierXDp / hypot;
    };
  }

  // Legendre Gauss Integrator
  private static float integrate(FloatFunction f) {
    float sum = 0f;
    for (int i = 0; i < ABSCISSAS_3.length; ++i) {
      sum += WEIGHTS_3[i] * f.evaluate(ABSCISSAS_3[i]);
    }
    return sum;
  }

  private interface FloatFunction {

    float evaluate(float v);
  }

  static class X implements Force {

    @Override
    public float getVertex(Edge edge) {
      Variable s = edge.getEnd();
      Variable sc = edge.getEndCtrl();
      Variable ec = edge.getStartCtrl();
      Variable e = edge.getStart();

      return integrate(derStartX(s.x, s.y, sc.x, sc.y, ec.x, ec.y, e.x, e.y));
    }

    @Override
    public float getCtrl(Edge edge) {
      Variable s = edge.getEnd();
      Variable sc = edge.getEndCtrl();
      Variable ec = edge.getStartCtrl();
      Variable e = edge.getStart();

      return integrate(derStartCtrlX(s.x, s.y, sc.x, sc.y, ec.x, ec.y, e.x, e.y));
    }
  }

  static class Y implements Force {

    @Override
    public float getVertex(Edge edge) {
      Variable s = edge.getEnd();
      Variable sc = edge.getEndCtrl();
      Variable ec = edge.getStartCtrl();
      Variable e = edge.getStart();

      return integrate(derStartX(s.y, s.x, sc.y, sc.x, ec.y, ec.x, e.y, e.x));
    }

    @Override
    public float getCtrl(Edge edge) {
      Variable s = edge.getEnd();
      Variable sc = edge.getEndCtrl();
      Variable ec = edge.getStartCtrl();
      Variable e = edge.getStart();

      return integrate(derStartCtrlX(s.y, s.x, sc.y, sc.x, ec.y, ec.x, e.y, e.x));
    }
  }
}
