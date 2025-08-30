package com.pippop.graphics.program;

import android.content.Context;
import android.opengl.GLES20;
import com.pippop.R;
import java.nio.FloatBuffer;

public class TextureProgram extends GraphicsProgram {

  private final int posHandle;
  private final int matrixHandle;
  private final int textureCoorHandle;
  private final int textureHandle;

  public TextureProgram(Context context) {
    super(context, R.raw.texture_fragment_shader, R.raw.texture_vertex_shader);
    this.posHandle = GLES20.glGetAttribLocation(program, "vPosition");
    this.matrixHandle = GLES20.glGetUniformLocation(program, "uMVPMatrix");
    this.textureCoorHandle = GLES20.glGetAttribLocation(program, "vTexCoordinate");
    this.textureHandle = GLES20.glGetUniformLocation(program, "uTexture");
  }

  public void draw(FloatBuffer buffer, int textureId, float[] transformMatrix) {
    GLES20.glUseProgram(program);

    GLES20.glEnable(GLES20.GL_BLEND);
    GLES20.glBlendFunc(GLES20.GL_SRC_ALPHA, GLES20.GL_ONE_MINUS_SRC_ALPHA);

    GLES20.glActiveTexture(GLES20.GL_TEXTURE0);
    GLES20.glBindTexture(GLES20.GL_TEXTURE_2D, textureId);
    GLES20.glUniform1i(textureHandle, 0);
    GLES20.glUniformMatrix2fv(matrixHandle, 1, false, transformMatrix, 0);

    buffer.position(0);
    GLES20.glEnableVertexAttribArray(posHandle);
    GLES20.glVertexAttribPointer(posHandle, 2, GLES20.GL_FLOAT, false, 16, buffer);

    buffer.position(2);
    GLES20.glEnableVertexAttribArray(textureCoorHandle);
    GLES20.glVertexAttribPointer(textureCoorHandle, 2, GLES20.GL_FLOAT, false, 16, buffer);

    GLES20.glDrawArrays(GLES20.GL_TRIANGLE_FAN, 0, 4);

    GLES20.glDisableVertexAttribArray(posHandle);
    GLES20.glDisableVertexAttribArray(textureCoorHandle);
    GLES20.glDisable(GLES20.GL_BLEND);
  }
}
