package com.pippop.graphics.gltext;

import android.graphics.Bitmap;
import android.opengl.GLES20;
import android.opengl.GLUtils;

class TextureHelper {

  static int loadTexture(Bitmap bitmap) {
    final int[] textureHandle = new int[1];

    GLES20.glGenTextures(1, textureHandle, 0);

    if (textureHandle[0] == 0) {
      throw new RuntimeException("Error loading texture.");
    }
    GLES20.glBindTexture(GLES20.GL_TEXTURE_2D, textureHandle[0]);

    // Set filtering
    GLES20.glTexParameteri(GLES20.GL_TEXTURE_2D, GLES20.GL_TEXTURE_MIN_FILTER, GLES20.GL_LINEAR);
    GLES20.glTexParameteri(GLES20.GL_TEXTURE_2D, GLES20.GL_TEXTURE_MAG_FILTER, GLES20.GL_LINEAR);
    GLES20.glTexParameterf(
        GLES20.GL_TEXTURE_2D, GLES20.GL_TEXTURE_WRAP_S, GLES20.GL_CLAMP_TO_EDGE); // Set U Wrapping
    GLES20.glTexParameterf(
        GLES20.GL_TEXTURE_2D, GLES20.GL_TEXTURE_WRAP_T, GLES20.GL_CLAMP_TO_EDGE); // Set V Wrapping

    // Load the bitmap into the bound texture.
    GLUtils.texImage2D(GLES20.GL_TEXTURE_2D, 0, bitmap, 0);

    // Recycle the bitmap, since its data has been loaded into OpenGL.
    bitmap.recycle();

    return textureHandle[0];
  }
}
