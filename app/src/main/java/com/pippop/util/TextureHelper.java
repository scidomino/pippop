package com.pippop.util;

import android.graphics.Bitmap;
import android.opengl.GLES20;
import android.opengl.GLUtils;

public class TextureHelper {

  public static int loadTexture(Bitmap bitmap) {
    final int[] textureHandle = new int[1];
    GLES20.glGenTextures(1, textureHandle, 0);
    if (textureHandle[0] == 0) {
      throw new RuntimeException("Error loading texture.");
    }
    GLES20.glBindTexture(GLES20.GL_TEXTURE_2D, textureHandle[0]);

    GLES20.glTexParameteri(GLES20.GL_TEXTURE_2D, GLES20.GL_TEXTURE_MIN_FILTER, GLES20.GL_LINEAR);
    GLES20.glTexParameteri(GLES20.GL_TEXTURE_2D, GLES20.GL_TEXTURE_MAG_FILTER, GLES20.GL_LINEAR);
    GLES20.glTexParameterf(GLES20.GL_TEXTURE_2D, GLES20.GL_TEXTURE_WRAP_S, GLES20.GL_CLAMP_TO_EDGE);
    GLES20.glTexParameterf(GLES20.GL_TEXTURE_2D, GLES20.GL_TEXTURE_WRAP_T, GLES20.GL_CLAMP_TO_EDGE);

    GLUtils.texImage2D(GLES20.GL_TEXTURE_2D, 0, bitmap, 0);

    bitmap.recycle();

    return textureHandle[0];
  }
}
