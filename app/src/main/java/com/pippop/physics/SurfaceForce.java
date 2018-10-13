package com.pippop.physics;

import com.pippop.graph.Edge;
import com.pippop.graph.Variable;

public class SurfaceForce {

  private static final DerStartX derSX = new DerStartX();
  private static final DerStartCtrlX derSCX = new DerStartCtrlX();

  public static class X implements Force {

    @Override
    public float getVertex(Edge edge) {
      Variable s = edge.getEnd();
      Variable sc = edge.getEndCtrl();
      Variable ec = edge.getStartCtrl();
      Variable e = edge.getStart();

      derSX.set(s.x, s.y, sc.x, sc.y, ec.x, ec.y, e.x, e.y);
      return derSX.integrate();
    }

    @Override
    public float getCtrl(Edge edge) {
      Variable s = edge.getEnd();
      Variable sc = edge.getEndCtrl();
      Variable ec = edge.getStartCtrl();
      Variable e = edge.getStart();

      derSCX.set(s.x, s.y, sc.x, sc.y, ec.x, ec.y, e.x, e.y);
      return derSCX.integrate();
    }
  }

  public static class Y implements Force {

    @Override
    public float getVertex(Edge edge) {
      Variable s = edge.getEnd();
      Variable sc = edge.getEndCtrl();
      Variable ec = edge.getStartCtrl();
      Variable e = edge.getStart();

      derSX.set(s.y, s.x, sc.y, sc.x, ec.y, ec.x, e.y, e.x);
      return derSX.integrate();
    }

    @Override
    public float getCtrl(Edge edge) {
      Variable s = edge.getEnd();
      Variable sc = edge.getEndCtrl();
      Variable ec = edge.getStartCtrl();
      Variable e = edge.getStart();

      derSCX.set(s.y, s.x, sc.y, sc.x, ec.y, ec.x, e.y, e.x);
      return derSCX.integrate();
    }
  }

  private static class DerStartX extends LegendreGaussIntegral {
    private float ax;
    private float ay;
    private float bx;
    private float by;
    private float cx;
    private float cy;

    void set(float sx, float sy, float scx, float scy, float ecx, float ecy, float ex, float ey) {
      ax = 3 * (scx - ecx) + ex - sx;
      ay = 3 * (scy - ecy) + ey - sy;
      bx = 2 * (sx - 2 * scx + ecx);
      by = 2 * (sy - 2 * scy + ecy);
      cx = scx - sx;
      cy = scy - sy;
    }

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
  }

  private static class DerStartCtrlX extends LegendreGaussIntegral {
    private float ax;
    private float ay;
    private float bx;
    private float by;
    private float cx;
    private float cy;

    void set(float sx, float sy, float scx, float scy, float ecx, float ecy, float ex, float ey) {
      ax = 3 * (scx - ecx) + ex - sx;
      ay = 3 * (scy - ecy) + ey - sy;
      bx = 2 * (sx - 2 * scx + ecx);
      by = 2 * (sy - 2 * scy + ecy);
      cx = scx - sx;
      cy = scy - sy;
    }

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
  }

  private abstract static class LegendreGaussIntegral {

    private static final float[] ABSCISSAS_3 = {
      (float) (.5 * (1 - Math.sqrt(3.0 / 5.0))), .5f, (float) (.5 * (1 + Math.sqrt(3.0 / 5.0)))
    };

    private static final float[] WEIGHTS_3 = {5f / 18f, 4f / 9f, 5f / 18f};

    float integrate() {
      float sum = 0f;
      for (int i = 0; i < ABSCISSAS_3.length; ++i) {
        sum += WEIGHTS_3[i] * evaluate(ABSCISSAS_3[i]);
      }
      return sum;
    }

    protected abstract float evaluate(float v);
  }
}
