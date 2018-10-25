package com.pippop.graphics.program;

import android.content.Context;
import android.opengl.GLES20;
import com.pippop.R;
import com.pippop.graphics.Color;
import java.nio.FloatBuffer;

public class GlowProgram extends GraphicsProgram {

  private final int colorHandle;
  private final int posHandle;
  private final int alphaHandle;
  private final int matrixHandle;

  public GlowProgram(Context context) {
    super(context, R.raw.standard_fragment_shader, R.raw.standard_vertex_shader);
    this.colorHandle = GLES20.glGetUniformLocation(program, "uColor");
    this.posHandle = GLES20.glGetAttribLocation(program, "vPosition");
    this.alphaHandle = GLES20.glGetAttribLocation(program, "vAlpha");
    this.matrixHandle = GLES20.glGetUniformLocation(program, "uMVPMatrix");
  }

  public void draw(FloatBuffer buffer, Color color, float[] transformMatrix) {
    GLES20.glEnable(GLES20.GL_BLEND);
    GLES20.glBlendFunc(GLES20.GL_SRC_ALPHA, GLES20.GL_ONE_MINUS_SRC_ALPHA);

    GLES20.glUseProgram(program);
    GLES20.glUniform4fv(colorHandle, 1, color.value, 0);
    GLES20.glUniformMatrix2fv(matrixHandle, 1, false, transformMatrix, 0);

    buffer.position(0);
    GLES20.glVertexAttribPointer(posHandle, 2, GLES20.GL_FLOAT, false, 12, buffer);
    GLES20.glEnableVertexAttribArray(posHandle);

    buffer.position(2);
    GLES20.glVertexAttribPointer(alphaHandle, 1, GLES20.GL_FLOAT, false, 12, buffer);
    GLES20.glEnableVertexAttribArray(alphaHandle);

    GLES20.glDrawArrays(GLES20.GL_TRIANGLE_STRIP, 0, buffer.limit() / 3);

    GLES20.glDisableVertexAttribArray(posHandle);
    GLES20.glDisableVertexAttribArray(alphaHandle);
  }
}
