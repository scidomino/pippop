package com.pippop.graphics.program;

import android.content.Context;
import android.opengl.GLES20;
import com.pippop.R;
import com.pippop.graphics.Color;
import java.nio.FloatBuffer;

public class StandardProgram extends GraphicsProgram {

  private final int colorHandle;
  private final int posHandle;
  private final int matrixHandle;

  public StandardProgram(Context context) {
    super(context, R.raw.standard_fragment_shader, R.raw.standard_vertex_shader);
    this.colorHandle = GLES20.glGetUniformLocation(program, "uColor");
    this.posHandle = GLES20.glGetAttribLocation(program, "vPosition");
    this.matrixHandle = GLES20.glGetUniformLocation(program, "uMVPMatrix");
  }

  public void draw(
      FloatBuffer buffer, Color color, int mode, int start, int endClip, float[] transformMatrix) {
    if (color.getAlpha() != 1.0) {
      GLES20.glEnable(GLES20.GL_BLEND);
      GLES20.glBlendFunc(GLES20.GL_SRC_ALPHA, GLES20.GL_ONE_MINUS_SRC_ALPHA);
    }
    GLES20.glUseProgram(program);
    GLES20.glUniform4fv(colorHandle, 1, color.value, 0);
    GLES20.glUniformMatrix2fv(matrixHandle, 1, false, transformMatrix, 0);
    GLES20.glEnableVertexAttribArray(posHandle);
    GLES20.glVertexAttribPointer(posHandle, 2, GLES20.GL_FLOAT, false, 0, buffer);
    GLES20.glDrawArrays(mode, start, buffer.limit() / 2 - endClip);
    GLES20.glDisableVertexAttribArray(posHandle);
    if (color.getAlpha() != 1) {
      GLES20.glDisable(GLES20.GL_BLEND);
    }
  }
}
