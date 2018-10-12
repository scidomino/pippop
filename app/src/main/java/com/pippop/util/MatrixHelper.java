package com.pippop.util;

/** Created by L on 10/5/2016. */
public class MatrixHelper {
  public static void perspectiveM(float[] m) {
    m[0] = 1f;
    m[1] = 0f;
    m[2] = 0f;
    m[3] = 0f;

    m[4] = 0f;
    m[5] = 1f;
    m[6] = 0f;
    m[7] = 0f;

    m[8] = 0f;
    m[9] = 0f;
    m[10] = 1f;
    m[11] = -1f;

    m[12] = 0f;
    m[13] = 0f;
    m[14] = 0f;
    m[15] = 0f;
  }
}
