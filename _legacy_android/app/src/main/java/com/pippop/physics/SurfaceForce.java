package com.pippop.physics;

import com.pippop.graph.Edge;
import com.pippop.graph.Variable;

class SurfaceForce {

  private static FloatFunction derStartX(
      float sx, float sy, float scx, float scy, float ecx, float ecy, float ex, float ey) {
    final float ax = 3 * (ecx - scx) + sx - ex;
    final float ay = 3 * (ecy - scy) + sy - ey;
    final float bx = 2 * (ex - 2 * ecx + scx);
    final float by = 2 * (ey - 2 * ecy + scy);
    final float cx = ecx - ex;
    final float cy = ecy - ey;

    return new FloatFunction() {
      @Override
      public float evaluate(float p) {
        float bezierXDp = cx + p * (bx + p * ax);
        float bezierYDp = cy + p * (by + p * ay);

        float hypot = (float) Math.hypot(bezierXDp, bezierYDp);
        if (hypot == 0) {
          return 0;
        }
        float bezierYDpDStartX = 3 * p * p;
        return bezierYDpDStartX * bezierXDp / hypot;
      }
    };
  }

  private static FloatFunction derStartCtrlX(
      float sx, float sy, float scx, float scy, float ecx, float ecy, float ex, float ey) {
    final float ax = 3 * (ecx - scx) + sx - ex;
    final float ay = 3 * (ecy - scy) + sy - ey;
    final float bx = 2 * (ex - 2 * ecx + scx);
    final float by = 2 * (ey - 2 * ecy + scy);
    final float cx = ecx - ex;
    final float cy = ecy - ey;

    return new FloatFunction() {
      @Override
      public float evaluate(float p) {
        float bezierXDp = cx + p * (bx + p * ax);
        float bezierYDp = cy + p * (by + p * ay);

        float hypot = (float) Math.hypot(bezierXDp, bezierYDp);
        if (hypot == 0) {
          return 0;
        }
        float bezierYDpDStartCtrlX = p * (6 - 9 * p);
        return bezierYDpDStartCtrlX * bezierXDp / hypot;
      }
    };
  }

  // 3 point Legendre Gauss Integrator
  private static float integrate(FloatFunction f) {
    return 0.44444444444f * f.evaluate(.5f)
        + 0.27777777777f * (f.evaluate(0.11270166537f) + f.evaluate(0.88729833462f));
  }

  // 5 point Legendre Gauss Integrator
  //  private static float integrate5(FloatFunction f) {
  //    return 0.28444444444f * f.evaluate(.5f)
  //        + 0.23931433525f * (f.evaluate(0.23076534494F) + f.evaluate(0.76923465505f))
  //        + 0.11846344252f * (f.evaluate(0.04691007703f) + f.evaluate(0.95308992296f));
  //  }

  private interface FloatFunction {
    float evaluate(float v);
  }

  static class X implements Force {

    @Override
    public float getVertex(Edge edge) {
      Variable s = edge.getStart();
      Variable sc = edge.getStartCtrl();
      Variable ec = edge.getEndCtrl();
      Variable e = edge.getEnd();

      return integrate(derStartX(s.x, s.y, sc.x, sc.y, ec.x, ec.y, e.x, e.y));
    }

    @Override
    public float getCtrl(Edge edge) {
      Variable s = edge.getStart();
      Variable sc = edge.getStartCtrl();
      Variable ec = edge.getEndCtrl();
      Variable e = edge.getEnd();

      return integrate(derStartCtrlX(s.x, s.y, sc.x, sc.y, ec.x, ec.y, e.x, e.y));
    }
  }

  static class Y implements Force {

    @Override
    public float getVertex(Edge edge) {
      Variable s = edge.getStart();
      Variable sc = edge.getStartCtrl();
      Variable ec = edge.getEndCtrl();
      Variable e = edge.getEnd();

      return integrate(derStartX(s.y, s.x, sc.y, sc.x, ec.y, ec.x, e.y, e.x));
    }

    @Override
    public float getCtrl(Edge edge) {
      Variable s = edge.getStart();
      Variable sc = edge.getStartCtrl();
      Variable ec = edge.getEndCtrl();
      Variable e = edge.getEnd();

      return integrate(derStartCtrlX(s.y, s.x, sc.y, sc.x, ec.y, ec.x, e.y, e.x));
    }
  }
}
