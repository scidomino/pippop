package com.pippop;

import android.content.Context;
import android.graphics.Bitmap;
import android.opengl.GLES20;
import android.opengl.GLSurfaceView;
import android.util.AttributeSet;
import com.pippop.graphics.Graphics;
import javax.microedition.khronos.egl.EGLConfig;
import javax.microedition.khronos.opengles.GL10;

public class TestGLSurfaceView extends GLSurfaceView {

  private Graphics graphics;

  public TestGLSurfaceView(Context context, AttributeSet attrs) {
    super(context, attrs);

    setEGLContextClientVersion(2);
    setRenderer(new GameRenderer());
    setRenderMode(GLSurfaceView.RENDERMODE_WHEN_DIRTY);
    setDebugFlags(DEBUG_CHECK_GL_ERROR | DEBUG_LOG_GL_CALLS);
  }

  private class GameRenderer implements Renderer {
    private Bitmap bitmap;

    @Override
    public void onSurfaceCreated(GL10 gl, EGLConfig config) {
      GLES20.glClearColor(1.0f, 0.0f, 0.0f, 1.0f);
      graphics = new Graphics(getContext());
    }

    public void onSurfaceChanged(GL10 gl, int w, int h) {
      graphics.updateDimensions(w, h);
    }

    public void onDrawFrame(GL10 gl) {
      GLES20.glClear(GLES20.GL_COLOR_BUFFER_BIT);
      GLES20.glEnable(GLES20.GL_TEXTURE_2D);
      //      graphics.drawTexture(bitmap, 0,0);
    }
  }
}
