package com.pippop.util;

import java.nio.FloatBuffer;

/**
 * User: Tommaso Sciortino Date: Jan 1, 2012 Time: 4:44:44 PM
 */
public class Tweener {

  public static void tween(
      FloatBuffer bigBuffer, FloatBuffer smallBuffer, FloatBuffer out, float morph) {
    if (smallBuffer.limit() > bigBuffer.limit()) {
      tween(smallBuffer, bigBuffer, out, 1 - morph);
      return;
    }

    int bigCount = bigBuffer.limit() / 2;
    int smallCount = bigBuffer.limit() / 2;

    float ratio = bigCount / (float) smallCount;
    float invM = 1 - morph;

    out.clear();
    out.put(invM * bigBuffer.get(0) + morph * bigBuffer.get(0));
    out.put(invM * bigBuffer.get(1) + morph * bigBuffer.get(1));

    for (int i = 1; i < smallCount - 1; i++) {
      int j = (int) (i * ratio);
      out.put(invM * bigBuffer.get(2 * j) + morph * bigBuffer.get(2 * i));
      out.put(invM * bigBuffer.get(2 * j + 1) + morph * bigBuffer.get(2 * i + 1));
    }
    out.put(out.get(2));
    out.put(out.get(3));

    out.flip();
  }
}
