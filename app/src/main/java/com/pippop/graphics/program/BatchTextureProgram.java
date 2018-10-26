package com.pippop.graphics.program;

import android.content.Context;
import android.opengl.GLES20;
import com.pippop.R;
import com.pippop.graphics.Color;
import java.nio.FloatBuffer;
import java.nio.ShortBuffer;

public class BatchTextureProgram extends GraphicsProgram {

  private static final int POSITION_CNT_2D = 2; // Number of Components in Vertex Color
  private static final int TEXCOORD_CNT = 2; // Number of Components in Vertex Normal
  private static final int MVP_MATRIX_INDEX_CNT = 1; // Number of Components in MVP matrix index
  private static final int VERTEX_SIZE =
      (POSITION_CNT_2D + TEXCOORD_CNT + MVP_MATRIX_INDEX_CNT) * 4;

  private final int textureCoorHandle;
  private final int posHandle;
  private final int matrixIndexHandle;
  private final int matrixHandle;
  private final int colorHandle;
  private final int textureHandle;

  public BatchTextureProgram(Context context) {
    super(context, R.raw.batch_texture_fragment_shader, R.raw.batch_texture_vertex_shader);
    matrixHandle = GLES20.glGetUniformLocation(program, "u_MVPMatrix");
    textureCoorHandle = GLES20.glGetAttribLocation(program, "a_TexCoordinate");
    matrixIndexHandle = GLES20.glGetAttribLocation(program, "a_MVPMatrixIndex");
    posHandle = GLES20.glGetAttribLocation(program, "a_Position");
    colorHandle = GLES20.glGetUniformLocation(program, "u_Color");
    textureHandle = GLES20.glGetUniformLocation(program, "u_Texture");
  }

  public void end(
      int numSprites, float[] uMVPMatrices, FloatBuffer vertices, ShortBuffer indices) {

    GLES20.glUniformMatrix4fv(matrixHandle, numSprites, false, uMVPMatrices, 0);
    GLES20.glEnableVertexAttribArray(matrixHandle);

    GLES20.glVertexAttribPointer(
        posHandle, POSITION_CNT_2D, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
    GLES20.glEnableVertexAttribArray(posHandle);

    vertices.position(POSITION_CNT_2D);
    GLES20.glVertexAttribPointer(
        textureCoorHandle, TEXCOORD_CNT, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
    GLES20.glEnableVertexAttribArray(textureCoorHandle);

    vertices.position(POSITION_CNT_2D + TEXCOORD_CNT);
    GLES20.glVertexAttribPointer(
        matrixIndexHandle, MVP_MATRIX_INDEX_CNT, GLES20.GL_FLOAT, false, VERTEX_SIZE, vertices);
    GLES20.glEnableVertexAttribArray(matrixIndexHandle);

    GLES20.glDrawElements(GLES20.GL_TRIANGLES, numSprites * 6, GLES20.GL_UNSIGNED_SHORT, indices);

    GLES20.glDisableVertexAttribArray(textureCoorHandle);
    GLES20.glDisableVertexAttribArray(colorHandle);
  }

  public void start(Color color, int textureId) {
    GLES20.glUseProgram(program);
    GLES20.glUniform4fv(colorHandle, 1, color.value, 0);
    GLES20.glEnableVertexAttribArray(colorHandle);
    GLES20.glActiveTexture(GLES20.GL_TEXTURE0);
    GLES20.glBindTexture(GLES20.GL_TEXTURE_2D, textureId);
    GLES20.glUniform1i(textureHandle, 0);
  }
}
