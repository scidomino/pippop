package com.pippop.util;

import static android.opengl.GLES20.GL_LINEAR;
import static android.opengl.GLES20.GL_LINEAR_MIPMAP_LINEAR;
import static android.opengl.GLES20.GL_TEXTURE_2D;
import static android.opengl.GLES20.GL_TEXTURE_MAG_FILTER;
import static android.opengl.GLES20.GL_TEXTURE_MIN_FILTER;
import static android.opengl.GLES20.glBindTexture;
import static android.opengl.GLES20.glDeleteTextures;
import static android.opengl.GLES20.glEnable;
import static android.opengl.GLES20.glGenTextures;
import static android.opengl.GLES20.glGenerateMipmap;
import static android.opengl.GLES20.glTexParameteri;
import static android.opengl.GLUtils.texImage2D;

import android.content.Context;
import android.graphics.Bitmap;
import android.graphics.BitmapFactory;
import android.util.Log;

/**
 * This method will read png and load data into OpenGL. Returns texture ID or 0. Created by L on
 * 10/5/2016.
 */
public class TextureHelper {
  // returns id of loaded texture
  public static int loadTexture(Context context, int resourceId) {
    // generate new texture
    final int[] textureObjectIds = new int[1];
    glGenTextures(1, textureObjectIds, 0);

    if (textureObjectIds[0] == 0) {
      Log.d("generate texture", "failed");
      return 0;
    }

    final BitmapFactory.Options options = new BitmapFactory.Options();
    // want original, not scaled
    options.inScaled = false;

    final Bitmap bitmap = BitmapFactory.decodeResource(context.getResources(), resourceId, options);

    if (bitmap == null) {
      Log.d("bitmap", "not decoded");
      glDeleteTextures(1, textureObjectIds, 0);
      return 0;
    }
    glEnable(GL_TEXTURE_2D);
    // this is 2D texture bound to ObjectId[x]
    glBindTexture(GL_TEXTURE_2D, textureObjectIds[0]);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);

    // load the bitmap data into OpenGL
    // copy data to texture object currently bound
    texImage2D(GL_TEXTURE_2D, 0, bitmap, 0);

    // no longer need the bitmap
    bitmap.recycle();

    glGenerateMipmap(GL_TEXTURE_2D);
    // texture is done loading; unbind it so we do not
    // accidentally modify it
    // 0 will unbind
    glBindTexture(GL_TEXTURE_2D, 0);
    Log.d("texture ID", Integer.toString(textureObjectIds[0]));
    return textureObjectIds[0];
  }
}
