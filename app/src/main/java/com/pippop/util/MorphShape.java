package com.pippop.util;

import com.pippop.graphics.Polygon;
import java.nio.FloatBuffer;

/** User: Tommaso Sciortino Date: Jan 1, 2012 Time: 4:44:44 PM */
public class MorphShape extends Polygon {

  public MorphShape() {
    super(100);
  }

  public void build(Polygon bigPolygon, Polygon smallPolygon, float morph) {
    if (smallPolygon.getVertices().limit() > bigPolygon.getVertices().limit()) {
      build(smallPolygon, bigPolygon, 1 - morph);
      return;
    }

    FloatBuffer big = bigPolygon.getVertices();
    FloatBuffer small = smallPolygon.getVertices();

    int bigCount = big.limit() / 2;
    int smallCount = small.limit() / 2;

    float ratio = bigCount / (float) smallCount;
    float invM = 1 - morph;

    FloatBuffer buffer = getVertices();
    buffer.clear();
    buffer.put(invM * big.get(0) + morph * small.get(0));
    buffer.put(invM * big.get(1) + morph * small.get(1));

    for (int i = 1; i < smallCount - 1; i++) {
      int j = (int) (i * ratio);
      buffer.put(invM * big.get(2 * j) + morph * small.get(2 * i));
      buffer.put(invM * big.get(2 * j + 1) + morph * small.get(2 * i + 1));
    }
    buffer.put(buffer.get(2));
    buffer.put(buffer.get(3));

    buffer.flip();
  }
}
